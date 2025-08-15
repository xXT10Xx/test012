use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub max_file_size_mb: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                base_url: "https://api.example.com".to_string(),
                timeout_seconds: 30,
                retry_attempts: 3,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: None,
            },
            storage: StorageConfig {
                data_dir: PathBuf::from("./data"),
                max_file_size_mb: 100,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut settings = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("RCLI"))
            .set_default("server.base_url", "https://api.example.com")?
            .set_default("server.timeout_seconds", 30)?
            .set_default("server.retry_attempts", 3)?
            .set_default("logging.level", "info")?
            .set_default("storage.data_dir", "./data")?
            .set_default("storage.max_file_size_mb", 100)?;

        if let Some(config_dir) = dirs::config_dir() {
            let app_config = config_dir.join("rcli").join("config");
            settings = settings.add_source(config::File::from(app_config).required(false));
        }

        let config = settings.build()?.try_deserialize()?;
        Ok(config)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }
}