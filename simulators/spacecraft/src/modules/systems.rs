use crate::modules::telemetry::{
    ComponentTelemetry,
    Data,
    DataValue,
};

/// Defines the behavior shared by spacecraft components.
pub trait Component {
    fn produce_data(&self) -> ComponentTelemetry;
}

/// Represents the spacecraft power system.
pub struct PowerSystem {
    pub name: String,
    pub battery_capacity_wh: f32,
    pub battery_energy_wh: f32,
    pub consumed_power_w: f32,
    pub battery_temperature: f32,
    pub solar_array_generating_power: bool,
}

impl Component for PowerSystem {
    /// Produces the telemetry data for the power system.
    fn produce_data(&self) -> ComponentTelemetry {
        ComponentTelemetry {
            component_name: self.name.clone(),
            data: vec![
                Data {
                    name: "state_of_charge".to_string(),
                    value: DataValue::Float(self.battery_voltage()),
                },
                Data {
                    name: "temperature".to_string(),
                    value: DataValue::Float(self.battery_temperature),
                },
            ],
        }
    }
}

const MIN_BATTERY_VOLTAGE: f32 = 24.0;
const MAX_BATTERY_VOLTAGE: f32 = 28.0;
const SOLAR_ARRAY_GENERATED_POWER_W: f32 = 600.0;

impl PowerSystem {
    /// Update the battery voltage (in a dumb way).
    pub fn update(&mut self, dt_seconds: f32) {
        let generated_power_w = if self.solar_array_generating_power { SOLAR_ARRAY_GENERATED_POWER_W } else {0.0};
        let net_power = generated_power_w - self.consumed_power_w;
        let energy_change = net_power * dt_seconds / 3600.0;
        self.battery_energy_wh = (self.battery_energy_wh + energy_change).clamp(0.0, self.battery_capacity_wh);
    }

    pub fn state_of_charge(&self) -> f32 {
        self.battery_energy_wh / self.battery_capacity_wh
    }

    fn battery_voltage(&self) -> f32 {
        MIN_BATTERY_VOLTAGE + self.state_of_charge() * (MAX_BATTERY_VOLTAGE - MIN_BATTERY_VOLTAGE)
    }
}