use crate::modules::anomalies::Anomaly;
use crate::modules::component::Component;
use crate::modules::events::{ComponentEvent, Event, EventType, Severity};
use crate::modules::telemetry::{ComponentTelemetry, Data, DataValue};

const MIN_BATTERY_VOLTAGE: f32 = 24.0;
const MAX_BATTERY_VOLTAGE: f32 = 28.0;
const SOLAR_ARRAY_GENERATED_POWER_W: f32 = 600.0;
const LOW_BATTERY_THRESHOLD: f32 = 0.40;
const CRITICAL_BATTERY_THRESHOLD: f32 = 0.20;
const MAX_BATTERY_TEMPERATURE: f32 = 50.0;

/// Represents the spacecraft power system.
pub struct PowerSystem {
    // todo(valid input data, e.g. positive inputs)
    pub name: String,
    pub battery_capacity_wh: f32,
    pub battery_energy_wh: f32,
    pub consumed_power_w: f32,
    pub battery_temperature: f32,
    pub solar_array_generating_power: bool,
    events: Vec<Event>,
}

impl PowerSystem {
    /// Creates a power system with an initially empty event queue.
    pub fn new(
        name: String,
        battery_capacity_wh: f32,
        battery_energy_wh: f32,
        consumed_power_w: f32,
        battery_temperature: f32,
        solar_array_generating_power: bool,
    ) -> Self {
        Self {
            name,
            battery_capacity_wh,
            battery_energy_wh,
            consumed_power_w,
            battery_temperature,
            solar_array_generating_power,
            events: Vec::new(),
        }
    }

    /// Returns the battery state of charge as a value between 0.0 and 1.0.
    pub fn state_of_charge(&self) -> f32 {
        self.battery_energy_wh / self.battery_capacity_wh
    }

    /// Calculates the battery voltage from its current state of charge.
    fn battery_voltage(&self) -> f32 {
        MIN_BATTERY_VOLTAGE + self.state_of_charge() * (MAX_BATTERY_VOLTAGE - MIN_BATTERY_VOLTAGE)
    }

    /// Creates an event originating from the power system.
    fn create_event(&self, event_type: EventType, severity: Severity, message: &str) -> Event {
        Event::new(self.name.clone(), event_type, severity, message.to_string())
    }
}

impl Component for PowerSystem {
    /// Updates battery energy based on power consumption and generation.
    fn update(&mut self, dt_seconds: f32) {
        let generated_power_w = if self.solar_array_generating_power {
            SOLAR_ARRAY_GENERATED_POWER_W
        } else {
            0.0
        };
        let net_power = generated_power_w - self.consumed_power_w;
        let energy_change = net_power * dt_seconds / 3600.0;
        self.battery_energy_wh =
            (self.battery_energy_wh + energy_change).clamp(0.0, self.battery_capacity_wh);
    }

    /// Produces the telemetry data for the power system.
    fn produce_telemetry(&self) -> ComponentTelemetry {
        ComponentTelemetry {
            component_name: self.name.clone(),
            data: vec![
                Data {
                    name: "battery_voltage".to_string(),
                    value: DataValue::Float(self.battery_voltage()),
                },
                Data {
                    name: "temperature".to_string(),
                    value: DataValue::Float(self.battery_temperature),
                },
            ],
        }
    }

    /// Derives the anomaly of the power system based on the state of charge.
    fn check_health(&mut self) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        let state_of_charge = self.state_of_charge();

        if state_of_charge <= CRITICAL_BATTERY_THRESHOLD {
            anomalies.push(Anomaly::BatteryCritical);
            let message = "battery critical";
            let event = self.create_event(
                EventType::Component(ComponentEvent::LowBatteryVoltage),
                Severity::Warning,
                message,
            );
            self.events.push(event)
        } else if state_of_charge <= LOW_BATTERY_THRESHOLD {
            anomalies.push(Anomaly::BatteryLow);
        }

        if self.battery_temperature >= MAX_BATTERY_TEMPERATURE {
            anomalies.push(Anomaly::BatteryOverheating);
            let message = "battery overheating";
            let event = self.create_event(
                EventType::Component(ComponentEvent::HighTemperature),
                Severity::Warning,
                message,
            );
            self.events.push(event)
        }

        anomalies
    }

    /// Returns all pending events and clears the event queue.
    fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
}
