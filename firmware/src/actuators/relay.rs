use std::time::{Duration, Instant};

use rppal::gpio::{Gpio, OutputPin};
use tracing::{info, warn};

pub struct Relay {
    pin: OutputPin,
    max_duration: Duration,
    cooldown: Duration,
    activated_at: Option<Instant>,
    deactivated_at: Option<Instant>,
}

impl Relay {
    pub fn new(gpio_pin: u8, max_duration_secs: u64, cooldown_secs: u64) -> anyhow::Result<Self> {
        let gpio = Gpio::new()?;
        let mut pin = gpio.get(gpio_pin)?.into_output();

        // Active LOW: set HIGH = relay OFF
        pin.set_high();

        Ok(Self {
            pin,
            max_duration: Duration::from_secs(max_duration_secs),
            cooldown: Duration::from_secs(cooldown_secs),
            activated_at: None,
            deactivated_at: None,
        })
    }

    pub fn activate(&mut self) -> anyhow::Result<()> {
        // Check cooldown
        if let Some(deactivated) = self.deactivated_at {
            let elapsed = deactivated.elapsed();
            if elapsed < self.cooldown {
                let remaining = self.cooldown - elapsed;
                anyhow::bail!(
                    "Relay cooldown active: {:.0}s remaining",
                    remaining.as_secs_f64()
                );
            }
        }

        // Active LOW: set LOW = relay ON
        self.pin.set_low();
        self.activated_at = Some(Instant::now());
        info!("Relay activated (pump ON)");
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.pin.set_high(); // Active LOW: HIGH = OFF
        self.deactivated_at = Some(Instant::now());
        self.activated_at = None;
        info!("Relay deactivated (pump OFF)");
    }

    pub fn is_active(&self) -> bool {
        self.activated_at.is_some()
    }

    /// Check if max duration exceeded and force deactivate if so.
    /// Call this periodically from the main loop.
    pub fn check_safety(&mut self) {
        if let Some(activated) = self.activated_at {
            if activated.elapsed() > self.max_duration {
                warn!(
                    "Relay safety cutoff: max duration {}s exceeded, forcing OFF",
                    self.max_duration.as_secs()
                );
                self.deactivate();
            }
        }
    }
}

impl Drop for Relay {
    fn drop(&mut self) {
        // Ensure relay is OFF on cleanup
        self.pin.set_high();
    }
}
