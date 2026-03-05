use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use super::ads1115::SharedAds1115;
use super::{Reading, Sensor, SensorType};

const ADC_CHANNEL: u8 = 0; // A0
const VALID_MOISTURE_MIN: f64 = 0.0;
const VALID_MOISTURE_MAX: f64 = 100.0;

// Calibration: voltage at dry (air) and wet (water)
const V_DRY: f64 = 3.3; // ~3.3V when completely dry
const V_WET: f64 = 1.0; // ~1.0V when submerged in water

pub struct MoistureSensor {
    adc: Arc<SharedAds1115>,
}

impl MoistureSensor {
    pub fn new(adc: Arc<SharedAds1115>) -> Self {
        Self { adc }
    }
}

#[async_trait]
impl Sensor for MoistureSensor {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        let voltage = self.adc.read_adc_channel(ADC_CHANNEL).await?;

        // Higher voltage = drier, lower voltage = wetter
        // Linear mapping: V_DRY → 0%, V_WET → 100%
        let moisture = ((V_DRY - voltage) / (V_DRY - V_WET) * 100.0).clamp(0.0, 100.0);

        debug!(
            "Moisture: voltage={:.4}V, moisture={:.1}%",
            voltage, moisture
        );

        if moisture < VALID_MOISTURE_MIN || moisture > VALID_MOISTURE_MAX {
            anyhow::bail!(
                "Moisture reading out of range: {:.1}% (valid: {}..{})",
                moisture,
                VALID_MOISTURE_MIN,
                VALID_MOISTURE_MAX
            );
        }

        Ok(vec![Reading::new(SensorType::Moisture, moisture, "%", 0)])
    }

    fn name(&self) -> &str {
        "YL-69 Moisture"
    }
}
