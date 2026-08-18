use crate::modules::anomalies::PowerAnomaly;
use crate::modules::mode::Mode;
use crate::modules::power_system::PowerSystem;

pub struct Spacecraft {
    pub id: String,
    pub power_system: PowerSystem,
    mode: Mode,
}

impl Spacecraft {
    pub fn new(id: String, power_system: PowerSystem) -> Self {
        Self {
            id,
            power_system,
            mode: Mode::Nominal,
        }
    }

    /// Returns the mode of the spacecraft.
    pub fn mode (&self) -> Mode {self.mode}

    /// Derives the spacecraft mode from the power anomaly.
    pub fn evaluate_autonomous_rules(&mut self) {
        match self.power_system.anomaly() {
            Some(PowerAnomaly::BatteryCritical) => {
                self.mode = Mode::Safe
            }
            Some(PowerAnomaly::BatteryLow) => {
                self.mode = Mode::Degraded
            }
            None => {}
        }
    }

    pub fn enter_safe_mode(&mut self) {
        self.mode = Mode::Safe;
    }
}