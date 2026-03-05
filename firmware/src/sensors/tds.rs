use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use super::ads1115::SharedAds1115;
use super::{Reading, Sensor, SensorType};

const ADC_CHANNEL: u8 = 2; // A2
const VALID_TDS_MIN: f64 = 0.0;
const VALID_TDS_MAX: f64 = 1000.0;
const VALID_EC_MIN: f64 = 0.0;
const VALID_EC_MAX: f64 = 2.0;

pub struct TdsSensor {
    adc: Arc<SharedAds1115>,
}

impl TdsSensor {
    pub fn new(adc: Arc<SharedAds1115>) -> Self {
        Self { adc }
    }

    pub async fn read_with_temp(&self, solution_temp: f64) -> anyhow::Result<(f64, f64)> {
        let voltage = self.adc.read_adc_channel(ADC_CHANNEL).await?;

        // Temperature compensation
        let coeff = 1.0 + 0.02 * (solution_temp - 25.0);
        let cv = voltage / coeff;

        // TDS formula
        let tds = (133.42 * cv.powi(3) - 255.86 * cv.powi(2) + 857.39 * cv) * 0.5;

        // EC = TDS * 2 / 1000 mS/cm
        let ec = tds * 2.0 / 1000.0;

        debug!(
            "TDS: voltage={:.4}V, cv={:.4}V, temp={:.1}°C, tds={:.0}ppm, ec={:.3}mS/cm",
            voltage, cv, solution_temp, tds, ec
        );

        if tds < VALID_TDS_MIN || tds > VALID_TDS_MAX {
            anyhow::bail!("TDS reading out of range: {:.0}ppm (valid: {}..{})", tds, VALID_TDS_MIN, VALID_TDS_MAX);
        }
        if ec < VALID_EC_MIN || ec > VALID_EC_MAX {
            anyhow::bail!("EC reading out of range: {:.3}mS/cm (valid: {}..{})", ec, VALID_EC_MIN, VALID_EC_MAX);
        }

        Ok((tds, ec))
    }
}

#[async_trait]
impl Sensor for TdsSensor {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        let (tds, ec) = self.read_with_temp(25.0).await?;
        Ok(vec![
            Reading::new(SensorType::TDS, tds, "ppm", 0),
            Reading::new(SensorType::EC, ec, "mS/cm", 0),
        ])
    }

    fn name(&self) -> &str {
        "TDS Sensor"
    }
}
