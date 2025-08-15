use crate::{AppError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredItem {
    pub id: String,
    pub key: String,
    pub value: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl StoredItem {
    pub fn new(key: String, value: Value) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            key,
            value,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    pub fn update_value(&mut self, value: Value) {
        self.value = value;
        self.updated_at = Utc::now();
    }
}

pub struct Storage {
    data_dir: PathBuf,
    max_file_size_mb: u64,
}

impl Storage {
    pub fn new(data_dir: PathBuf, max_file_size_mb: u64) -> Result<Self> {
        fs::create_dir_all(&data_dir)?;
        
        Ok(Self {
            data_dir,
            max_file_size_mb,
        })
    }

    fn get_file_path(&self, key: &str) -> PathBuf {
        let safe_key = key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        self.data_dir.join(format!("{}.json", safe_key))
    }

    pub async fn store(&self, key: String, value: Value) -> Result<StoredItem> {
        let file_path = self.get_file_path(&key);
        
        let item = if file_path.exists() {
            let mut existing_item = self.get(&key).await?;
            existing_item.update_value(value);
            existing_item
        } else {
            StoredItem::new(key, value)
        };

        let json_data = serde_json::to_string_pretty(&item)?;
        
        if json_data.len() > (self.max_file_size_mb * 1024 * 1024) as usize {
            return Err(AppError::Validation {
                message: format!("Data size exceeds maximum allowed size of {} MB", self.max_file_size_mb),
            });
        }

        fs::write(&file_path, json_data)?;
        info!("Stored item with key: {}", item.key);
        
        Ok(item)
    }

    pub async fn get(&self, key: &str) -> Result<StoredItem> {
        let file_path = self.get_file_path(key);
        
        if !file_path.exists() {
            return Err(AppError::NotFound {
                resource: format!("key '{}'", key),
            });
        }

        let json_data = fs::read_to_string(&file_path)?;
        let item: StoredItem = serde_json::from_str(&json_data)?;
        
        debug!("Retrieved item with key: {}", key);
        Ok(item)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        
        for entry in fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(stem) = path.file_stem() {
                    if let Some(key) = stem.to_str() {
                        keys.push(key.to_string());
                    }
                }
            }
        }
        
        keys.sort();
        debug!("Listed {} keys", keys.len());
        Ok(keys)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let file_path = self.get_file_path(key);
        
        if !file_path.exists() {
            return Err(AppError::NotFound {
                resource: format!("key '{}'", key),
            });
        }

        fs::remove_file(&file_path)?;
        info!("Deleted item with key: {}", key);
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> bool {
        self.get_file_path(key).exists()
    }

    pub async fn get_metadata(&self, key: &str) -> Result<HashMap<String, String>> {
        let item = self.get(key).await?;
        Ok(item.metadata)
    }

    pub fn get_storage_info(&self) -> Result<StorageInfo> {
        let mut total_size = 0u64;
        let mut file_count = 0u32;

        for entry in fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(metadata) = fs::metadata(&path) {
                    total_size += metadata.len();
                    file_count += 1;
                }
            }
        }

        Ok(StorageInfo {
            data_dir: self.data_dir.clone(),
            file_count,
            total_size_bytes: total_size,
            max_file_size_mb: self.max_file_size_mb,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct StorageInfo {
    pub data_dir: PathBuf,
    pub file_count: u32,
    pub total_size_bytes: u64,
    pub max_file_size_mb: u64,
}