use crate::modules::component::Component;
use crate::modules::events::ComponentEvent;
use crate::modules::telemetry::{ComponentTelemetry, Data, DataValue};

const MIN_BATTERY_VOLTAGE: f32 = 24.0;
const MAX_BATTERY_VOLTAGE: f32 = 28.0;
const SOLAR_ARRAY_GENERATED_POWER_W: f32 = 600.0;
const LOW_BATTERY_THRESHOLD: f32 = 0.40;
const CRITICAL_BATTERY_THRESHOLD: f32 = 0.20;
const MAX_BATTERY_TEMPERATURE: f32 = 50.0;

/// Represents the current battery health state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatteryHealth {
    Nominal,
    Low,
    Critical,
}

/// Represents the spacecraft power system.
pub struct PowerSystem {
    // todo(valid input data, e.g. positive inputs)
    pub name: String,

    // physical state
    pub battery_capacity_wh: f32,
    pub battery_energy_wh: f32,
    pub consumed_power_w: f32,
    pub battery_temperature: f32,
    pub solar_array_generating_power: bool,

    // health state
    battery_overheating: bool,
    battery_health: BatteryHealth,
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
            battery_overheating: false,
            battery_health: BatteryHealth::Nominal,
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

    /// Checks if the battery level is lower than the critical threshold.
    fn is_battery_critical(&self) -> bool {
        self.state_of_charge() <= CRITICAL_BATTERY_THRESHOLD
    }

    /// Checks if the battery level is lower than the low threshold.
    fn is_battery_low(&self) -> bool {
        self.state_of_charge() <= LOW_BATTERY_THRESHOLD
    }

    /// Checks if the battery level is higher than the high temperature threshold.
    fn is_battery_temperature_high(&self) -> bool {
        self.battery_temperature >= MAX_BATTERY_TEMPERATURE
    }

    /// Derives the current battery health state.
    fn battery_health(&self) -> BatteryHealth {
        if self.is_battery_critical() {
            BatteryHealth::Critical
        } else if self.is_battery_low() {
            BatteryHealth::Low
        } else {
            BatteryHealth::Nominal
        }
    }

    pub fn current_battery_health(&self) -> BatteryHealth {
        self.battery_health
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

    /// Derives component events based on anomalies and health state.
    fn evaluate_health(&mut self) -> Vec<ComponentEvent> {
        let mut events = Vec::new();

        let new_battery_health = self.battery_health();

        if new_battery_health != self.battery_health {
            match (self.battery_health, new_battery_health) {
                (BatteryHealth::Nominal, BatteryHealth::Low) => {
                    events.push(ComponentEvent::BatteryLowDetected);
                }

                (BatteryHealth::Low, BatteryHealth::Critical) => {
                    events.push(ComponentEvent::BatteryCriticalDetected);
                }

                (BatteryHealth::Critical, BatteryHealth::Low) => {
                    events.push(ComponentEvent::BatteryCriticalCleared);
                }

                (BatteryHealth::Low, BatteryHealth::Nominal) => {
                    events.push(ComponentEvent::BatteryLowCleared);
                }

                _ => {}
            }

            self.battery_health = new_battery_health;
        }

        let overheating = self.is_battery_temperature_high();

        if overheating && !self.battery_overheating {
            events.push(ComponentEvent::HighTemperatureDetected);
            self.battery_overheating = true;
        }

        if !overheating && self.battery_overheating {
            events.push(ComponentEvent::HighTemperatureCleared);
            self.battery_overheating = false;
        }

        events
    }
}
