use std::path::PathBuf;

use async_trait::async_trait;
use tokio::fs;
use tracing::{debug, warn};

use super::{Reading, Sensor, SensorType};

const W1_DEVICES_PATH: &str = "/sys/bus/w1/devices";
const VALID_TEMP_MIN: f64 = -10.0;
const VALID_TEMP_MAX: f64 = 85.0;

pub struct Ds18b20 {
    device_path: Option<PathBuf>,
}

impl Ds18b20 {
    pub async fn new() -> anyhow::Result<Self> {
        let device_path = Self::find_device().await?;
        Ok(Self { device_path })
    }

    async fn find_device() -> anyhow::Result<Option<PathBuf>> {
        let devices_dir = PathBuf::from(W1_DEVICES_PATH);
        if !devices_dir.exists() {
            warn!("W1 devices directory not found — DS18B20 not available");
            return Ok(None);
        }

        let mut entries = fs::read_dir(&devices_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("28-") {
                let temp_file = entry.path().join("temperature");
                if temp_file.exists() {
                    debug!("Found DS18B20 device: {}", name_str);
                    return Ok(Some(temp_file));
                }
            }
        }

        warn!("No DS18B20 device found in {}", W1_DEVICES_PATH);
        Ok(None)
    }

    async fn read_temperature(&self) -> anyhow::Result<f64> {
        let path = self
            .device_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("DS18B20 device not found"))?;

        let content = fs::read_to_string(path).await?;
        let millidegrees: f64 = content.trim().parse()?;
        let temp = millidegrees / 1000.0;

        if temp < VALID_TEMP_MIN || temp > VALID_TEMP_MAX {
            anyhow::bail!(
                "DS18B20 reading out of range: {:.1}°C (valid: {}..{})",
                temp,
                VALID_TEMP_MIN,
                VALID_TEMP_MAX
            );
        }

        Ok(temp)
    }
}

#[async_trait]
impl Sensor for Ds18b20 {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        let temp = self.read_temperature().await?;
        Ok(vec![Reading::new(
            SensorType::SolutionTemp,
            temp,
            "°C",
            0,
        )])
    }

    fn name(&self) -> &str {
        "DS18B20"
    }
}
