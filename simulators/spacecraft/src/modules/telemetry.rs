use crate::modules::mode::Mode;

use serde::Serialize;

/// Represents a value contained in a telemetry data point.
#[derive(Debug, Serialize)]
pub enum DataValue {
    Float(f32),
    Boolean(bool),
    Integer(i32),
    Text(String),
}

/// Represents a single telemetry data point.
#[derive(Debug, Serialize)]
pub struct Data {
    pub name: String,
    pub value: DataValue,
}

/// Represents the telemetry produced by a spacecraft component.
#[derive(Debug, Serialize)]
pub struct ComponentTelemetry {
    pub component_name: String,
    pub data: Vec<Data>,
}

/// Represents a complete spacecraft telemetry packet.
#[derive(Debug, Serialize)]
pub struct SpacecraftTelemetry {
    pub simulation_id: String,
    pub spacecraft_id: String,
    pub mode: Mode,
    pub components: Vec<ComponentTelemetry>,
}