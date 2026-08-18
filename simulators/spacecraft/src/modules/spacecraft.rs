use crate::modules::anomalies::Anomaly;
use crate::modules::component::Component;
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
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Derives the spacecraft mode from the power anomaly.
    pub fn evaluate_autonomous_rules(&mut self) {
        let anomalies = self.power_system.check_health();

        if anomalies.contains(&Anomaly::BatteryCritical) {
            self.mode = Mode::Safe;
        } else if anomalies.contains(&Anomaly::BatteryLow) {
            self.mode = Mode::Degraded;
        }
    }

    pub fn enter_safe_mode(&mut self) {
        self.mode = Mode::Safe;
    }
}
