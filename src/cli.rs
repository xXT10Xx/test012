use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rcli")]
#[command(about = "An advanced CLI tool demonstrating Rust best practices")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Fetch data from a remote API")]
    Fetch {
        #[arg(help = "URL to fetch data from")]
        url: String,
        
        #[arg(short, long, help = "Output format")]
        format: Option<OutputFormat>,
        
        #[arg(short, long, help = "Save response to file")]
        output: Option<PathBuf>,
    },
    
    #[command(about = "Store data locally")]
    Store {
        #[arg(help = "Key to store data under")]
        key: String,
        
        #[arg(help = "Value to store (JSON string or file path)")]
        value: String,
        
        #[arg(short, long, help = "Treat value as file path")]
        file: bool,
    },
    
    #[command(about = "Retrieve stored data")]
    Get {
        #[arg(help = "Key to retrieve")]
        key: String,
        
        #[arg(short, long, help = "Output format")]
        format: Option<OutputFormat>,
    },
    
    #[command(about = "List all stored keys")]
    List {
        #[arg(short, long, help = "Show detailed information")]
        detailed: bool,
    },
    
    #[command(about = "Delete stored data")]
    Delete {
        #[arg(help = "Key to delete")]
        key: String,
    },
    
    #[command(about = "Generate configuration file")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    #[command(about = "Generate default configuration file")]
    Init {
        #[arg(short, long, help = "Output path for config file")]
        output: Option<PathBuf>,
    },
    
    #[command(about = "Show current configuration")]
    Show,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    Yaml,
    Pretty,
}