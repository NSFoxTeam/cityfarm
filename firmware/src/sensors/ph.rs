use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use super::ads1115::SharedAds1115;
use super::{Reading, Sensor, SensorType};
use crate::calibration::CalibrationData;

const ADC_CHANNEL: u8 = 1; // A1
const VALID_PH_MIN: f64 = 0.0;
const VALID_PH_MAX: f64 = 14.0;

pub struct PhSensor {
    adc: Arc<SharedAds1115>,
    calibration: CalibrationData,
}

impl PhSensor {
    pub fn new(adc: Arc<SharedAds1115>, calibration: CalibrationData) -> Self {
        Self { adc, calibration }
    }

    pub fn update_calibration(&mut self, calibration: CalibrationData) {
        self.calibration = calibration;
    }

    pub async fn read_with_temp(&self, solution_temp: f64) -> anyhow::Result<f64> {
        let voltage = self.adc.read_adc_channel(ADC_CHANNEL).await?;

        // Nernst temperature compensation
        let v_comp = voltage + 0.003 * (solution_temp - 25.0);

        // Two-point calibration: pH = slope * V + offset
        let ph = self.calibration.slope * v_comp + self.calibration.offset;

        debug!(
            "pH: voltage={:.4}V, v_comp={:.4}V, temp={:.1}°C, pH={:.2}",
            voltage, v_comp, solution_temp, ph
        );

        if ph < VALID_PH_MIN || ph > VALID_PH_MAX {
            anyhow::bail!("pH reading out of range: {:.2} (valid: {}..{})", ph, VALID_PH_MIN, VALID_PH_MAX);
        }

        Ok(ph)
    }
}

#[async_trait]
impl Sensor for PhSensor {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        // Default to 25°C if no temp available (call read_with_temp directly for compensation)
        let ph = self.read_with_temp(25.0).await?;
        Ok(vec![Reading::new(SensorType::PH, ph, "pH", 0)])
    }

    fn name(&self) -> &str {
        "pH Sensor"
    }
}
