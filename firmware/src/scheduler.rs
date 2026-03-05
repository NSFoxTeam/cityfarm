use std::sync::Arc;
use std::time::Duration;

use linux_embedded_hal::I2cdev;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::calibration::CalibrationData;
use crate::config::Config;
use crate::sensors::ads1115::SharedAds1115;
use crate::sensors::bh1750::Bh1750;
use crate::sensors::dht22::Dht22;
use crate::sensors::ds18b20::Ds18b20;
use crate::sensors::moisture::MoistureSensor;
use crate::sensors::ph::PhSensor;
use crate::sensors::tds::TdsSensor;
use crate::sensors::Reading;
use crate::transport::buffer::SqliteBuffer;
use crate::transport::http::HttpTransport;

pub async fn run(config: Config) -> anyhow::Result<()> {
    info!("Starting sensor scheduler (interval: {}s)", config.read_interval_secs);

    // Initialize I2C bus (shared)
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let bus = Arc::new(Mutex::new(i2c));

    // Initialize shared ADS1115
    let adc = Arc::new(SharedAds1115::new(bus.clone()));

    // Load calibration
    let calibration = CalibrationData::load(&config.calibration_path)?;

    // Initialize sensors
    let ds18b20 = match Ds18b20::new().await {
        Ok(s) => Some(s),
        Err(e) => {
            warn!("DS18B20 init failed (will skip): {}", e);
            None
        }
    };

    let ph_sensor = PhSensor::new(adc.clone(), calibration);
    let tds_sensor = TdsSensor::new(adc.clone());
    let dht22 = match Dht22::new() {
        Ok(s) => Some(s),
        Err(e) => {
            warn!("DHT22 init failed (will skip): {}", e);
            None
        }
    };
    let bh1750 = Bh1750::new(bus.clone());
    let moisture = MoistureSensor::new(adc.clone());

    // Initialize transport
    let buffer = SqliteBuffer::new(&config.db_path)?;
    let http = HttpTransport::new(&config.backend_url, &config.api_key)?;

    let interval = Duration::from_secs(config.read_interval_secs);
    let mut ticker = tokio::time::interval(interval);

    info!("Scheduler started, reading sensors every {}s", config.read_interval_secs);

    loop {
        ticker.tick().await;

        let mut batch: Vec<Reading> = Vec::new();

        // 1. DS18B20 — solution temperature (needed for pH/TDS compensation)
        let solution_temp = if let Some(ref ds) = ds18b20 {
            match ds.read_temperature().await {
                Ok(temp) => {
                    batch.push(Reading::new(
                        crate::sensors::SensorType::SolutionTemp,
                        temp,
                        "°C",
                        0,
                    ));
                    temp
                }
                Err(e) => {
                    warn!("DS18B20 read failed: {}", e);
                    25.0 // fallback
                }
            }
        } else {
            25.0
        };

        // 2. pH (with temperature compensation)
        match ph_sensor.read_with_temp(solution_temp).await {
            Ok(ph) => batch.push(Reading::new(crate::sensors::SensorType::PH, ph, "pH", 0)),
            Err(e) => warn!("pH read failed: {}", e),
        }

        // 3. TDS/EC (with temperature compensation)
        match tds_sensor.read_with_temp(solution_temp).await {
            Ok((tds, ec)) => {
                batch.push(Reading::new(crate::sensors::SensorType::TDS, tds, "ppm", 0));
                batch.push(Reading::new(crate::sensors::SensorType::EC, ec, "mS/cm", 0));
            }
            Err(e) => warn!("TDS read failed: {}", e),
        }

        // 4. DHT22 — air temperature + humidity
        if let Some(ref dht) = dht22 {
            match crate::sensors::Sensor::read(dht).await {
                Ok(readings) => batch.extend(readings),
                Err(e) => warn!("DHT22 read failed: {}", e),
            }
        }

        // 5. BH1750 — light levels
        match crate::sensors::Sensor::read(&bh1750).await {
            Ok(readings) => batch.extend(readings),
            Err(e) => warn!("BH1750 read failed: {}", e),
        }

        // 6. Moisture
        match crate::sensors::Sensor::read(&moisture).await {
            Ok(readings) => batch.extend(readings),
            Err(e) => warn!("Moisture read failed: {}", e),
        }

        if batch.is_empty() {
            warn!("No sensor readings collected this cycle");
            continue;
        }

        info!("Collected {} readings", batch.len());

        // Push to SQLite buffer
        if let Err(e) = buffer.push(&batch) {
            error!("Failed to buffer readings: {}", e);
            continue;
        }

        // Try to flush buffer to backend
        flush_buffer(&buffer, &http).await;
    }
}

async fn flush_buffer(buffer: &SqliteBuffer, http: &HttpTransport) {
    let entries = match buffer.pop(100) {
        Ok(e) => e,
        Err(e) => {
            warn!("Failed to pop from buffer: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        return;
    }

    let mut flushed_ids = Vec::new();

    for (id, payload) in &entries {
        match http.send_readings(payload).await {
            Ok(()) => {
                flushed_ids.push(*id);
            }
            Err(e) => {
                warn!("Failed to send batch {}: {}", id, e);
                break; // Stop on first failure, retry next cycle
            }
        }
    }

    if !flushed_ids.is_empty() {
        if let Err(e) = buffer.delete(&flushed_ids) {
            error!("Failed to delete flushed entries: {}", e);
        } else {
            info!("Flushed {} batches to backend", flushed_ids.len());
        }
    }

    match buffer.count() {
        Ok(count) if count > 0 => info!("{} batches still buffered", count),
        _ => {}
    }
}
