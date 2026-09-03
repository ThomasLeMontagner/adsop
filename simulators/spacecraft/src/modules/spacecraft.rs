use super::events::{EventType, EventsManager, ModeChangeEvent};
use super::mode::Mode;
use super::subsystems::Component;
use super::subsystems::power::{BatteryHealth, PowerSystem};
use super::telemetry::SpacecraftTelemetry;
use chrono::Utc;

/// Represents a spacecraft and its components, operating mode, and pending events.
pub struct Spacecraft {
    pub name: String,
    pub power_system: PowerSystem,
    mode: Mode,
    event_manager: EventsManager,
}

impl Spacecraft {
    /// Creates a spacecraft in nominal mode with an empty event queue.
    pub fn new(id: String, power_system: PowerSystem) -> Self {
        Self {
            name: id,
            power_system,
            mode: Mode::Nominal,
            event_manager: EventsManager::new(),
        }
    }

    /// Returns the current operating mode of the spacecraft.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Updates the components of the spacecraft.
    pub fn update(&mut self, dt_seconds: f32) {
        self.power_system.update(dt_seconds);
        self.collect_component_events();
        self.evaluate_autonomous_rules();
    }

    /// Evaluates component anomalies and updates the spacecraft mode accordingly.
    /// Requires explicit recovery logic / ground command for recovery.
    pub fn evaluate_autonomous_rules(&mut self) {
        let current_mode = self.mode;

        match self.power_system.current_battery_health() {
            BatteryHealth::Critical => self.mode = Mode::Safe,
            BatteryHealth::Low => self.mode = Mode::Degraded,
            BatteryHealth::Nominal => {}
        }

        if current_mode != self.mode {
            let mode_change_event = ModeChangeEvent::new(current_mode, self.mode);
            self.event_manager.add_event(
                self.name.clone(),
                EventType::ModeChange(mode_change_event),
                mode_change_event.severity(),
                mode_change_event.message(),
            )
        }
    }

    /// Sets the spacecraft operating mode to safe.
    pub fn enter_safe_mode(&mut self) {
        self.mode = Mode::Safe;
    }

    /// Moves component events into the spacecraft event manager.
    pub fn collect_component_events(&mut self) {
        for component_event in self.power_system.evaluate_health() {
            let severity = component_event.severity();
            let message = component_event.message().to_string();

            self.event_manager.add_event(
                self.power_system.name.clone(),
                EventType::Component(component_event),
                severity,
                message,
            );
        }
    }

    /// Produces spacecraft telemetry containing all events awaiting delivery confirmation.
    pub fn produce_telemetry(&self, simulation_id: &str) -> SpacecraftTelemetry {
        SpacecraftTelemetry {
            simulation_id: simulation_id.to_string(),
            spacecraft_id: self.name.clone(),
            mode: self.mode(),
            components: vec![self.power_system.produce_telemetry()],

            events: self.event_manager.get_events_to_transmit(),
            timestamp: Utc::now(),
        }
    }

    /// Records a transmission attempt for the supplied event identifiers.
    pub fn record_event_transmissions(&mut self, event_ids: &[u32]) {
        self.event_manager
            .record_transmissions(event_ids, Utc::now());
    }

    /// Removes delivered events from the pending queue.
    pub fn confirm_event_deliveries(&mut self, event_ids: &[u32]) {
        self.event_manager.confirm_deliveries(event_ids);
    }
}
