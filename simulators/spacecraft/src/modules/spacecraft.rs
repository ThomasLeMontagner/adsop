use crate::modules::anomalies::Anomaly;
use crate::modules::component::Component;
use crate::modules::events::{Event, EventType, ModeChangeEvent, Severity};
use crate::modules::mode::Mode;
use crate::modules::power_system::PowerSystem;
use crate::modules::telemetry::SpacecraftTelemetry;

/// Represents a spacecraft and its components, operating mode, and pending events.
pub struct Spacecraft {
    pub name: String,
    pub power_system: PowerSystem,
    mode: Mode,
    pub events: Vec<Event>,
}

impl Spacecraft {
    /// Creates a spacecraft in nominal mode with an empty event queue.
    pub fn new(id: String, power_system: PowerSystem) -> Self {
        Self {
            name: id,
            power_system,
            mode: Mode::Nominal,
            events: Vec::new(),
        }
    }

    /// Returns the current operating mode of the spacecraft.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Evaluates component anomalies and updates the spacecraft mode accordingly.
    pub fn evaluate_autonomous_rules(&mut self) {
        let anomalies = self.power_system.check_health();
        let current_mode = self.mode;

        if anomalies.contains(&Anomaly::BatteryCritical) {
            self.mode = Mode::Safe;
        } else if anomalies.contains(&Anomaly::BatteryLow) {
            self.mode = Mode::Degraded;
        }

        if current_mode != self.mode {
            let mode_change_event = ModeChangeEvent::new(current_mode, self.mode);

            let message = format!("Mode changed from {:?} to {:?}", current_mode, self.mode);
            let event = self.create_event(
                EventType::ModeChange(mode_change_event),
                Severity::Warning,
                &message,
            );
            self.events.push(event);
        }
    }

    /// Sets the spacecraft operating mode to safe.
    pub fn enter_safe_mode(&mut self) {
        self.mode = Mode::Safe;
    }

    /// Moves pending component events into the spacecraft event queue.
    pub fn collect_component_events(&mut self) {
        self.events.extend(self.power_system.take_events());
    }

    /// Produces spacecraft telemetry and clears all pending events.
    pub fn produce_telemetry(&mut self, simulation_id: &str) -> SpacecraftTelemetry {
        let power_system_telemetry = self.power_system.produce_telemetry();

        self.collect_component_events();

        SpacecraftTelemetry {
            simulation_id: simulation_id.to_string(),
            spacecraft_id: self.name.clone(),
            mode: self.mode(),
            components: vec![power_system_telemetry],

            // Move all events into telemetry and clear the spacecraft queue.
            events: std::mem::take(&mut self.events),
        }
    }

    /// Creates an event originating from the spacecraft.
    fn create_event(&self, event_type: EventType, severity: Severity, message: &str) -> Event {
        Event::new(
            self.name.clone(),
            event_type,
            severity,
            message.to_string(),
        )
    }
}
