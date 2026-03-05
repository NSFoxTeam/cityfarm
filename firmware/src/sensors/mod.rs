use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SensorType {
    Temperature,
    Humidity,
    Light,
    Moisture,
    PH,
    TDS,
    EC,
    SolutionTemp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: &'static str,
    pub level: u8,
    pub timestamp: DateTime<Utc>,
}

impl Reading {
    pub fn new(sensor_type: SensorType, value: f64, unit: &'static str, level: u8) -> Self {
        Self {
            sensor_type,
            value,
            unit,
            level,
            timestamp: Utc::now(),
        }
    }
}

#[async_trait]
pub trait Sensor: Send + Sync {
    async fn read(&self) -> anyhow::Result<Vec<Reading>>;
    fn name(&self) -> &str;
}

pub mod ads1115;
pub mod bh1750;
pub mod dht22;
pub mod ds18b20;
pub mod moisture;
pub mod ph;
pub mod tds;
