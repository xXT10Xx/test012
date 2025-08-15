use crate::config::LoggingConfig;
use crate::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let mut layers = Vec::new();
    layers.push(stdout_layer.boxed());

    if let Some(file_path) = &config.file_path {
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false);

        layers.push(file_layer.boxed());
    }

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layers)
        .init();

    Ok(())
}