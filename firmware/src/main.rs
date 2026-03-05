mod actuators;
mod calibration;
mod config;
mod scheduler;
mod sensors;
mod transport;

use std::sync::Arc;

use clap::{Parser, Subcommand};
use linux_embedded_hal::I2cdev;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use calibration::CalibrationData;
use config::Config;
use sensors::ads1115::SharedAds1115;
use sensors::Sensor;

#[derive(Parser)]
#[command(name = "cityfarm-agent", version, about = "CityFarm hydroponic IoT agent")]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "/opt/cityfarm/config.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run main sensor loop
    Run,
    /// Interactive pH calibration
    Calibrate {
        #[command(subcommand)]
        sensor: CalibrateTarget,
    },
    /// One-shot read of all sensors
    TestSensors,
}

#[derive(Subcommand)]
enum CalibrateTarget {
    /// Calibrate pH sensor (two-point: pH 7.0 and pH 4.0)
    Ph,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = Config::load(Some(&cli.config))?;

    match cli.command {
        Commands::Run => {
            info!("Starting cityfarm-agent");

            // Graceful shutdown on SIGTERM/SIGINT
            let shutdown = tokio::signal::ctrl_c();

            tokio::select! {
                result = scheduler::run(config) => {
                    if let Err(e) = result {
                        error!("Scheduler error: {}", e);
                    }
                }
                _ = shutdown => {
                    info!("Received shutdown signal, exiting");
                }
            }
        }
        Commands::Calibrate { sensor } => match sensor {
            CalibrateTarget::Ph => calibrate_ph(&config).await?,
        },
        Commands::TestSensors => test_sensors(&config).await?,
    }

    Ok(())
}

async fn calibrate_ph(config: &Config) -> anyhow::Result<()> {
    use std::io::{self, Write};

    let i2c = I2cdev::new("/dev/i2c-1")?;
    let bus = Arc::new(Mutex::new(i2c));
    let adc = Arc::new(SharedAds1115::new(bus));
    let cal = CalibrationData::default();
    let ph_sensor = sensors::ph::PhSensor::new(adc, cal);

    println!("=== pH Calibration (Two-Point) ===\n");

    // Step 1: pH 7.0 buffer
    println!("Place the pH probe in pH 7.0 buffer solution.");
    print!("Press Enter when ready...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("Taking 10 samples at pH 7.0...");
    let mut voltages_neutral = Vec::new();
    for i in 0..10 {
        // Read raw voltage (bypass pH calculation)
        let v = ph_sensor.read_with_temp(25.0).await;
        // We need raw voltage, so read ADC directly
        // For calibration, we read the raw ADC voltage
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // Re-read via the sensor's ADC
        match v {
            Ok(val) => {
                println!("  Sample {}: pH={:.3}", i + 1, val);
                voltages_neutral.push(val);
            }
            Err(e) => {
                println!("  Sample {} failed: {}", i + 1, e);
            }
        }
    }

    if voltages_neutral.is_empty() {
        anyhow::bail!("No successful readings at pH 7.0");
    }

    // For proper calibration we need raw voltages. Using a simplified approach:
    // Read raw ADC voltage directly
    let i2c2 = I2cdev::new("/dev/i2c-1")?;
    let bus2 = Arc::new(Mutex::new(i2c2));
    let adc2 = Arc::new(SharedAds1115::new(bus2));

    println!("\nReading raw voltages for calibration...");
    println!("Probe should still be in pH 7.0 buffer.");
    print!("Press Enter when ready...");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;

    let mut v_neutral_samples = Vec::new();
    for i in 0..10 {
        match adc2.read_adc_channel(1).await {
            Ok(v) => {
                println!("  Sample {}: {:.4}V", i + 1, v);
                v_neutral_samples.push(v);
            }
            Err(e) => println!("  Sample {} failed: {}", i + 1, e),
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    if v_neutral_samples.is_empty() {
        anyhow::bail!("No voltage readings at pH 7.0");
    }
    let v_neutral: f64 = v_neutral_samples.iter().sum::<f64>() / v_neutral_samples.len() as f64;
    println!("\nAverage voltage at pH 7.0: {:.4}V", v_neutral);

    // Step 2: pH 4.0 buffer
    println!("\nPlace the pH probe in pH 4.0 buffer solution.");
    print!("Press Enter when ready...");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;

    println!("Taking 10 samples at pH 4.0...");
    let mut v_acid_samples = Vec::new();
    for i in 0..10 {
        match adc2.read_adc_channel(1).await {
            Ok(v) => {
                println!("  Sample {}: {:.4}V", i + 1, v);
                v_acid_samples.push(v);
            }
            Err(e) => println!("  Sample {} failed: {}", i + 1, e),
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    if v_acid_samples.is_empty() {
        anyhow::bail!("No voltage readings at pH 4.0");
    }
    let v_acid: f64 = v_acid_samples.iter().sum::<f64>() / v_acid_samples.len() as f64;
    println!("Average voltage at pH 4.0: {:.4}V\n", v_acid);

    // Calculate calibration
    let cal_data = CalibrationData::from_two_point(v_neutral, v_acid);
    println!("Calibration results:");
    println!("  Slope: {:.4}", cal_data.slope);
    println!("  Offset: {:.4}", cal_data.offset);
    println!("  V_neutral: {:.4}V", v_neutral);
    println!("  V_acid: {:.4}V", v_acid);

    // Save
    cal_data.save(&config.calibration_path)?;
    println!("\nCalibration saved to {}", config.calibration_path);

    Ok(())
}

async fn test_sensors(config: &Config) -> anyhow::Result<()> {
    println!("=== Sensor Test ===\n");

    let i2c = I2cdev::new("/dev/i2c-1")?;
    let bus = Arc::new(Mutex::new(i2c));
    let adc = Arc::new(SharedAds1115::new(bus.clone()));

    let calibration = CalibrationData::load(&config.calibration_path)?;

    // DS18B20
    print!("DS18B20 (solution temp): ");
    let solution_temp = match sensors::ds18b20::Ds18b20::new().await {
        Ok(ds) => match ds.read_temperature().await {
            Ok(temp) => {
                println!("{:.1}°C", temp);
                temp
            }
            Err(e) => {
                println!("ERROR: {}", e);
                25.0
            }
        },
        Err(e) => {
            println!("NOT FOUND: {}", e);
            25.0
        }
    };

    // pH
    print!("pH: ");
    let ph_sensor = sensors::ph::PhSensor::new(adc.clone(), calibration);
    match ph_sensor.read_with_temp(solution_temp).await {
        Ok(ph) => println!("{:.2} pH", ph),
        Err(e) => println!("ERROR: {}", e),
    }

    // TDS/EC
    print!("TDS/EC: ");
    let tds_sensor = sensors::tds::TdsSensor::new(adc.clone());
    match tds_sensor.read_with_temp(solution_temp).await {
        Ok((tds, ec)) => println!("{:.0} ppm / {:.3} mS/cm", tds, ec),
        Err(e) => println!("ERROR: {}", e),
    }

    // DHT22
    print!("DHT22 (air temp/humidity): ");
    match sensors::dht22::Dht22::new() {
        Ok(dht) => match Sensor::read(&dht).await {
            Ok(readings) => {
                for r in &readings {
                    print!("{:.1}{} ", r.value, r.unit);
                }
                println!();
            }
            Err(e) => println!("ERROR: {}", e),
        },
        Err(e) => println!("NOT AVAILABLE: {}", e),
    }

    // BH1750
    print!("BH1750 (light): ");
    let bh = sensors::bh1750::Bh1750::new(bus.clone());
    match Sensor::read(&bh).await {
        Ok(readings) => {
            for r in &readings {
                print!("L{}={:.0}{} ", r.level, r.value, r.unit);
            }
            println!();
        }
        Err(e) => println!("ERROR: {}", e),
    }

    // Moisture
    print!("Moisture: ");
    let moist = sensors::moisture::MoistureSensor::new(adc.clone());
    match Sensor::read(&moist).await {
        Ok(readings) => {
            for r in &readings {
                println!("{:.1}{}", r.value, r.unit);
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }

    println!("\n=== Test Complete ===");
    Ok(())
}
