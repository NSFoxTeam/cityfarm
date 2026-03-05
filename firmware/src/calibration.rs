use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationData {
    pub slope: f64,
    pub offset: f64,
    pub calibrated_at: Option<DateTime<Utc>>,
    pub v_neutral: Option<f64>,
    pub v_acid: Option<f64>,
}

impl Default for CalibrationData {
    fn default() -> Self {
        Self {
            slope: -5.70,
            offset: 21.34,
            calibrated_at: None,
            v_neutral: None,
            v_acid: None,
        }
    }
}

impl CalibrationData {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let path = Path::new(path);
        if !path.exists() {
            warn!("Calibration file not found at {}, using defaults", path.display());
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        let data: CalibrationData = serde_json::from_str(&content)?;
        info!("Loaded calibration from {}", path.display());
        Ok(data)
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        info!("Saved calibration to {}", path);
        Ok(())
    }

    pub fn from_two_point(v_neutral: f64, v_acid: f64) -> Self {
        // pH 7.0 at v_neutral, pH 4.0 at v_acid
        let slope = (4.0 - 7.0) / (v_acid - v_neutral);
        let offset = 7.0 - slope * v_neutral;

        Self {
            slope,
            offset,
            calibrated_at: Some(Utc::now()),
            v_neutral: Some(v_neutral),
            v_acid: Some(v_acid),
        }
    }
}
