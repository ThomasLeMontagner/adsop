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
    pub battery_voltage: f32,
    pub temperature: f32,
}

impl Component for PowerSystem {
    fn produce_data(&self) -> ComponentTelemetry {
        ComponentTelemetry {
            component_name: self.name.clone(),
            data: vec![
                Data {
                    name: "battery_voltage".to_string(),
                    value: DataValue::Float(self.battery_voltage),
                },
                Data {
                    name: "temperature".to_string(),
                    value: DataValue::Float(self.temperature),
                },
            ],
        }
    }
}