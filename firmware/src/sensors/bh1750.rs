use std::sync::Arc;

use async_trait::async_trait;
use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use tokio::sync::Mutex;
use tracing::{debug, warn};

use super::{Reading, Sensor, SensorType};

const TCA9548A_ADDR: u8 = 0x70;
const BH1750_ADDR: u8 = 0x23;

// BH1750 commands
const CMD_POWER_ON: u8 = 0x01;
const CMD_ONE_TIME_HIGH_RES: u8 = 0x20;

const VALID_LUX_MIN: f64 = 0.0;
const VALID_LUX_MAX: f64 = 65535.0;

// BH1750 on TCA9548A channels 0, 1, 2 (one per level)
const CHANNELS: &[(u8, u8)] = &[
    (0, 0), // channel 0, level 0
    (1, 1), // channel 1, level 1
    (2, 2), // channel 2, level 2
];

pub struct Bh1750 {
    bus: Arc<Mutex<I2cdev>>,
}

impl Bh1750 {
    pub fn new(bus: Arc<Mutex<I2cdev>>) -> Self {
        Self { bus }
    }

    async fn read_channel(&self, tca_channel: u8) -> anyhow::Result<f64> {
        let mut bus = self.bus.lock().await;

        // Select TCA9548A channel
        let tca_byte = 1u8 << tca_channel;
        I2c::write(&mut *bus, TCA9548A_ADDR, &[tca_byte])
            .map_err(|e| anyhow::anyhow!("TCA9548A select channel {}: {:?}", tca_channel, e))?;

        // Power on BH1750
        I2c::write(&mut *bus, BH1750_ADDR, &[CMD_POWER_ON])
            .map_err(|e| anyhow::anyhow!("BH1750 power on: {:?}", e))?;

        // Start one-time high resolution measurement
        I2c::write(&mut *bus, BH1750_ADDR, &[CMD_ONE_TIME_HIGH_RES])
            .map_err(|e| anyhow::anyhow!("BH1750 start measurement: {:?}", e))?;

        // Wait for measurement (max 180ms for high-res mode)
        drop(bus);
        tokio::time::sleep(std::time::Duration::from_millis(180)).await;
        let mut bus = self.bus.lock().await;

        // Re-select TCA channel after releasing lock
        I2c::write(&mut *bus, TCA9548A_ADDR, &[tca_byte])
            .map_err(|e| anyhow::anyhow!("TCA9548A re-select: {:?}", e))?;

        // Read 2 bytes
        let mut buf = [0u8; 2];
        I2c::read(&mut *bus, BH1750_ADDR, &mut buf)
            .map_err(|e| anyhow::anyhow!("BH1750 read: {:?}", e))?;

        let raw = ((buf[0] as u16) << 8) | buf[1] as u16;
        let lux = raw as f64 / 1.2;

        Ok(lux)
    }
}

#[async_trait]
impl Sensor for Bh1750 {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        let mut readings = Vec::new();

        for &(channel, level) in CHANNELS {
            match self.read_channel(channel).await {
                Ok(lux) => {
                    if lux < VALID_LUX_MIN || lux > VALID_LUX_MAX {
                        warn!("BH1750 ch{} lux out of range: {:.1}", channel, lux);
                        continue;
                    }
                    debug!("BH1750 ch{}: {:.1} lux", channel, lux);
                    readings.push(Reading::new(SensorType::Light, lux, "lux", level));
                }
                Err(e) => {
                    warn!("BH1750 ch{} read failed: {}", channel, e);
                }
            }
        }

        if readings.is_empty() {
            anyhow::bail!("All BH1750 channels failed");
        }

        Ok(readings)
    }

    fn name(&self) -> &str {
        "BH1750"
    }
}
