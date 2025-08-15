use clap::Parser;
use rust_advanced_cli::{
    cli::{Cli, Commands, ConfigAction, OutputFormat},
    config::AppConfig,
    http::HttpClient,
    logging,
    storage::Storage, Result,
};
use serde_json::Value;
use std::path::PathBuf;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    
    let config = if let Some(config_path) = &cli.config {
        AppConfig::load_from_file(config_path)?
    } else {
        AppConfig::load()?
    };

    if cli.verbose {
        let mut logging_config = config.logging.clone();
        logging_config.level = "debug".to_string();
        logging::init_logging(&logging_config)?;
    } else {
        logging::init_logging(&config.logging)?;
    }

    info!("Starting rust-advanced-cli");

    let http_client = HttpClient::new(
        config.server.base_url.clone(),
        config.server.timeout_seconds,
        config.server.retry_attempts,
    )?;

    let storage = Storage::new(
        config.storage.data_dir.clone(),
        config.storage.max_file_size_mb,
    )?;

    match cli.command {
        Commands::Fetch { url, format, output } => {
            handle_fetch(&http_client, &url, format, output).await?;
        }
        Commands::Store { key, value, file } => {
            handle_store(&storage, key, value, file).await?;
        }
        Commands::Get { key, format } => {
            handle_get(&storage, key, format).await?;
        }
        Commands::List { detailed } => {
            handle_list(&storage, detailed).await?;
        }
        Commands::Delete { key } => {
            handle_delete(&storage, key).await?;
        }
        Commands::Config { action } => {
            handle_config(action, &config).await?;
        }
    }

    info!("Operation completed successfully");
    Ok(())
}

async fn handle_fetch(
    client: &HttpClient,
    url: &str,
    format: Option<OutputFormat>,
    output: Option<PathBuf>,
) -> Result<()> {
    let data = client.fetch_json(url).await?;
    let formatted = format_output(&data, format.unwrap_or(OutputFormat::Pretty))?;

    if let Some(output_path) = output {
        std::fs::write(&output_path, &formatted)?;
        println!("Data saved to: {}", output_path.display());
    } else {
        println!("{}", formatted);
    }

    Ok(())
}

async fn handle_store(storage: &Storage, key: String, value: String, is_file: bool) -> Result<()> {
    let data: Value = if is_file {
        let file_content = std::fs::read_to_string(&value)?;
        serde_json::from_str(&file_content)?
    } else {
        serde_json::from_str(&value)?
    };

    let item = storage.store(key, data).await?;
    println!("Stored item with ID: {}", item.id);
    Ok(())
}

async fn handle_get(storage: &Storage, key: String, format: Option<OutputFormat>) -> Result<()> {
    let item = storage.get(&key).await?;
    let formatted = format_output(&item.value, format.unwrap_or(OutputFormat::Pretty))?;
    println!("{}", formatted);
    Ok(())
}

async fn handle_list(storage: &Storage, detailed: bool) -> Result<()> {
    let keys = storage.list().await?;
    
    if detailed {
        let storage_info = storage.get_storage_info()?;
        println!("Storage Information:");
        println!("  Directory: {}", storage_info.data_dir.display());
        println!("  Files: {}", storage_info.file_count);
        println!("  Total size: {} bytes", storage_info.total_size_bytes);
        println!("  Max file size: {} MB", storage_info.max_file_size_mb);
        println!();
    }

    if keys.is_empty() {
        println!("No stored items found.");
    } else {
        println!("Stored keys ({}):", keys.len());
        for key in keys {
            if detailed {
                if let Ok(item) = storage.get(&key).await {
                    println!("  {} (created: {}, updated: {})", 
                        key, 
                        item.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                        item.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                } else {
                    println!("  {} (error reading metadata)", key);
                }
            } else {
                println!("  {}", key);
            }
        }
    }
    Ok(())
}

async fn handle_delete(storage: &Storage, key: String) -> Result<()> {
    storage.delete(&key).await?;
    println!("Deleted key: {}", key);
    Ok(())
}

async fn handle_config(action: ConfigAction, config: &AppConfig) -> Result<()> {
    match action {
        ConfigAction::Init { output } => {
            let output_path = output.unwrap_or_else(|| PathBuf::from("config.yaml"));
            config.save_to_file(&output_path)?;
            println!("Configuration saved to: {}", output_path.display());
        }
        ConfigAction::Show => {
            let yaml = serde_yaml::to_string(config)?;
            println!("{}", yaml);
        }
    }
    Ok(())
}

fn format_output(data: &Value, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string(data)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(data)?),
        OutputFormat::Pretty => Ok(serde_json::to_string_pretty(data)?),
    }
}
