use std::sync::Arc;

use anyhow::Context;
use linux_embedded_hal::I2cdev;
use tokio::sync::Mutex;
use tracing::debug;

const TCA9548A_ADDR: u8 = 0x70;
const ADS1115_ADDR: u8 = 0x48;
const TCA_CHANNEL_ADS: u8 = 7;

// ADS1115 registers
const REG_CONVERSION: u8 = 0x00;
const REG_CONFIG: u8 = 0x01;

// Config bits for ADS1115
const CONFIG_OS_SINGLE: u16 = 0x8000; // Start single conversion
const CONFIG_PGA_4_096V: u16 = 0x0200; // +/-4.096V range
const CONFIG_MODE_SINGLE: u16 = 0x0100; // Single-shot mode
const CONFIG_DR_128SPS: u16 = 0x0080; // 128 samples per second
const CONFIG_COMP_DISABLE: u16 = 0x0003; // Disable comparator

// Mux channel configs (bits 14:12)
const CONFIG_MUX_AIN0: u16 = 0x4000; // AIN0 vs GND
const CONFIG_MUX_AIN1: u16 = 0x5000; // AIN1 vs GND
const CONFIG_MUX_AIN2: u16 = 0x6000; // AIN2 vs GND
const CONFIG_MUX_AIN3: u16 = 0x7000; // AIN3 vs GND

const PGA_VOLTAGE: f64 = 4.096;

pub struct SharedAds1115 {
    bus: Arc<Mutex<I2cdev>>,
}

impl SharedAds1115 {
    pub fn new(bus: Arc<Mutex<I2cdev>>) -> Self {
        Self { bus }
    }

    fn mux_config(adc_channel: u8) -> u16 {
        match adc_channel {
            0 => CONFIG_MUX_AIN0,
            1 => CONFIG_MUX_AIN1,
            2 => CONFIG_MUX_AIN2,
            3 => CONFIG_MUX_AIN3,
            _ => CONFIG_MUX_AIN0,
        }
    }

    pub async fn read_channel(&self, mux_channel: u8, adc_channel: u8) -> anyhow::Result<f64> {
        use embedded_hal::i2c::I2c;

        let mut bus = self.bus.lock().await;

        // Select TCA9548A channel
        let tca_byte = 1u8 << mux_channel;
        I2c::write(&mut *bus, TCA9548A_ADDR, &[tca_byte])
            .map_err(|e| anyhow::anyhow!("TCA9548A select channel {}: {:?}", mux_channel, e))?;

        // Configure ADS1115 for single-shot read
        let config = CONFIG_OS_SINGLE
            | Self::mux_config(adc_channel)
            | CONFIG_PGA_4_096V
            | CONFIG_MODE_SINGLE
            | CONFIG_DR_128SPS
            | CONFIG_COMP_DISABLE;

        let config_bytes = config.to_be_bytes();
        I2c::write(&mut *bus, ADS1115_ADDR, &[REG_CONFIG, config_bytes[0], config_bytes[1]])
            .map_err(|e| anyhow::anyhow!("ADS1115 write config: {:?}", e))?;

        // Wait for conversion (128 SPS → ~8ms, use 10ms)
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Poll until conversion complete
        let mut attempts = 0;
        loop {
            let mut config_buf = [0u8; 2];
            I2c::write_read(&mut *bus, ADS1115_ADDR, &[REG_CONFIG], &mut config_buf)
                .map_err(|e| anyhow::anyhow!("ADS1115 read config: {:?}", e))?;
            let status = u16::from_be_bytes(config_buf);
            if status & CONFIG_OS_SINGLE != 0 {
                break;
            }
            attempts += 1;
            if attempts > 10 {
                anyhow::bail!("ADS1115 conversion timeout");
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }

        // Read conversion result
        let mut result_buf = [0u8; 2];
        I2c::write_read(&mut *bus, ADS1115_ADDR, &[REG_CONVERSION], &mut result_buf)
            .map_err(|e| anyhow::anyhow!("ADS1115 read conversion: {:?}", e))?;

        let raw = i16::from_be_bytes(result_buf);
        let voltage = (raw as f64) * PGA_VOLTAGE / 32767.0;

        debug!(
            "ADS1115 mux_ch={} adc_ch={} raw={} voltage={:.4}V",
            mux_channel, adc_channel, raw, voltage
        );

        Ok(voltage)
    }

    /// Read voltage from ADS1115 on TCA9548A channel 7 (default)
    pub async fn read_adc_channel(&self, adc_channel: u8) -> anyhow::Result<f64> {
        self.read_channel(TCA_CHANNEL_ADS, adc_channel)
            .await
            .context(format!("Reading ADC channel {}", adc_channel))
    }
}
