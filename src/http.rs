use crate::{AppError, Result};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tracing::{info, warn};

pub struct HttpClient {
    client: Client,
    base_url: String,
    retry_attempts: u32,
}

impl HttpClient {
    pub fn new(base_url: String, timeout_seconds: u64, retry_attempts: u32) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .user_agent("rust-advanced-cli/0.1.0")
            .build()?;

        Ok(Self {
            client,
            base_url,
            retry_attempts,
        })
    }

    pub async fn fetch_json(&self, url: &str) -> Result<Value> {
        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}/{}", self.base_url.trim_end_matches('/'), url.trim_start_matches('/'))
        };

        info!("Fetching data from: {}", full_url);

        for attempt in 1..=self.retry_attempts {
            match self.client.get(&full_url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: Value = response.json().await?;
                        info!("Successfully fetched data (attempt {})", attempt);
                        return Ok(json);
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        
                        if attempt == self.retry_attempts {
                            return Err(AppError::OperationFailed {
                                reason: format!("HTTP {}: {}", status, error_text),
                            });
                        } else {
                            warn!("Request failed with status {} (attempt {}), retrying...", status, attempt);
                        }
                    }
                }
                Err(e) => {
                    if attempt == self.retry_attempts {
                        return Err(AppError::Http(e));
                    } else {
                        warn!("Request failed (attempt {}): {}, retrying...", attempt, e);
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
        }

        unreachable!()
    }

    pub async fn post_json(&self, url: &str, data: &Value) -> Result<Value> {
        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}/{}", self.base_url.trim_end_matches('/'), url.trim_start_matches('/'))
        };

        info!("Posting data to: {}", full_url);

        let response = self
            .client
            .post(&full_url)
            .json(data)
            .send()
            .await?;

        if response.status().is_success() {
            let json: Value = response.json().await?;
            info!("Successfully posted data");
            Ok(json)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AppError::OperationFailed {
                reason: format!("HTTP {}: {}", status, error_text),
            })
        }
    }
}