use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use rppal::gpio::{Gpio, IoPin, Mode};
use tokio::sync::Mutex;
use tracing::{debug, warn};

use super::{Reading, Sensor, SensorType};

const GPIO_PIN: u8 = 4;
const MIN_READ_INTERVAL: Duration = Duration::from_secs(2);
const VALID_TEMP_MIN: f64 = -40.0;
const VALID_TEMP_MAX: f64 = 80.0;
const VALID_HUMIDITY_MIN: f64 = 0.0;
const VALID_HUMIDITY_MAX: f64 = 100.0;

pub struct Dht22 {
    state: Arc<Mutex<Dht22State>>,
}

struct Dht22State {
    pin: IoPin,
    last_read: Option<Instant>,
}

impl Dht22 {
    pub fn new() -> anyhow::Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(GPIO_PIN)?.into_io(Mode::Input);

        Ok(Self {
            state: Arc::new(Mutex::new(Dht22State {
                pin,
                last_read: None,
            })),
        })
    }

    /// Read raw data from DHT22 using single-wire protocol
    fn read_raw(pin: &mut IoPin) -> anyhow::Result<(f64, f64)> {
        // Send start signal: pull low for 1ms, then high for 30us
        pin.set_mode(Mode::Output);
        pin.set_low();
        std::thread::sleep(Duration::from_millis(1));
        pin.set_high();
        std::thread::sleep(Duration::from_micros(30));
        pin.set_mode(Mode::Input);

        // Wait for DHT22 response (low ~80us, then high ~80us)
        Self::wait_for_level(pin, false, 100)?;
        Self::wait_for_level(pin, true, 100)?;
        Self::wait_for_level(pin, false, 100)?;

        // Read 40 bits (5 bytes)
        let mut data = [0u8; 5];
        for i in 0..40 {
            // Each bit starts with ~50us low
            Self::wait_for_level(pin, true, 100)?;

            // Measure high duration: ~26-28us = 0, ~70us = 1
            let start = Instant::now();
            Self::wait_for_level(pin, false, 100)?;
            let duration = start.elapsed();

            if duration > Duration::from_micros(40) {
                data[i / 8] |= 1 << (7 - (i % 8));
            }
        }

        // Verify checksum
        let checksum = (data[0] as u16 + data[1] as u16 + data[2] as u16 + data[3] as u16) & 0xFF;
        if checksum != data[4] as u16 {
            anyhow::bail!(
                "DHT22 checksum mismatch: calculated {} vs received {}",
                checksum,
                data[4]
            );
        }

        // Parse humidity (bytes 0-1) and temperature (bytes 2-3)
        let humidity = ((data[0] as u16) << 8 | data[1] as u16) as f64 / 10.0;
        let raw_temp = ((data[2] as u16 & 0x7F) << 8 | data[3] as u16) as f64 / 10.0;
        let temperature = if data[2] & 0x80 != 0 {
            -raw_temp
        } else {
            raw_temp
        };

        Ok((temperature, humidity))
    }

    fn wait_for_level(pin: &IoPin, high: bool, timeout_us: u64) -> anyhow::Result<()> {
        let start = Instant::now();
        let timeout = Duration::from_micros(timeout_us);
        loop {
            let level = pin.is_high();
            if level == high {
                return Ok(());
            }
            if start.elapsed() > timeout {
                anyhow::bail!("DHT22 timeout waiting for {} level", if high { "HIGH" } else { "LOW" });
            }
        }
    }
}

#[async_trait]
impl Sensor for Dht22 {
    async fn read(&self) -> anyhow::Result<Vec<Reading>> {
        let mut state = self.state.lock().await;

        // Enforce minimum interval between reads
        if let Some(last) = state.last_read {
            let elapsed = last.elapsed();
            if elapsed < MIN_READ_INTERVAL {
                let wait = MIN_READ_INTERVAL - elapsed;
                drop(state);
                tokio::time::sleep(wait).await;
                state = self.state.lock().await;
            }
        }

        // Try up to 3 times
        let mut last_err = None;
        for attempt in 0..3 {
            match Self::read_raw(&mut state.pin) {
                Ok((temp, humidity)) => {
                    state.last_read = Some(Instant::now());

                    if temp < VALID_TEMP_MIN || temp > VALID_TEMP_MAX {
                        anyhow::bail!("DHT22 temperature out of range: {:.1}°C", temp);
                    }
                    if humidity < VALID_HUMIDITY_MIN || humidity > VALID_HUMIDITY_MAX {
                        anyhow::bail!("DHT22 humidity out of range: {:.1}%", humidity);
                    }

                    debug!("DHT22: temp={:.1}°C, humidity={:.1}%", temp, humidity);

                    return Ok(vec![
                        Reading::new(SensorType::Temperature, temp, "°C", 0),
                        Reading::new(SensorType::Humidity, humidity, "%", 0),
                    ]);
                }
                Err(e) => {
                    warn!("DHT22 read attempt {} failed: {}", attempt + 1, e);
                    last_err = Some(e);
                    // Wait before retry
                    drop(state);
                    tokio::time::sleep(MIN_READ_INTERVAL).await;
                    state = self.state.lock().await;
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("DHT22 read failed")))
    }

    fn name(&self) -> &str {
        "DHT22"
    }
}
