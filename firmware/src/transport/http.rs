use std::time::Duration;

use reqwest::Client;
use tracing::{debug, warn};

const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);

pub struct HttpTransport {
    client: Client,
    url: String,
    api_key: String,
}

impl HttpTransport {
    pub fn new(backend_url: &str, api_key: &str) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let url = format!("{}/api/v1/readings", backend_url.trim_end_matches('/'));

        Ok(Self {
            client,
            url,
            api_key: api_key.to_string(),
        })
    }

    pub async fn send_readings(&self, payload: &str) -> anyhow::Result<()> {
        let mut backoff = INITIAL_BACKOFF;

        for attempt in 1..=MAX_RETRIES {
            match self
                .client
                .post(&self.url)
                .header("Content-Type", "application/json")
                .header("X-API-Key", &self.api_key)
                .body(payload.to_string())
                .send()
                .await
            {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        debug!("Sent readings to backend (attempt {})", attempt);
                        return Ok(());
                    }

                    let body = resp.text().await.unwrap_or_default();
                    warn!(
                        "Backend returned {} (attempt {}): {}",
                        status, attempt, body
                    );

                    if status.is_client_error() {
                        // Don't retry client errors (4xx)
                        anyhow::bail!("Backend rejected readings: {} {}", status, body);
                    }
                }
                Err(e) => {
                    warn!("HTTP request failed (attempt {}): {}", attempt, e);
                }
            }

            if attempt < MAX_RETRIES {
                tokio::time::sleep(backoff).await;
                backoff *= 2;
            }
        }

        anyhow::bail!("Failed to send readings after {} attempts", MAX_RETRIES)
    }
}
