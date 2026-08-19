use crate::modules::mode::Mode;

use serde::Serialize;
use crate::modules::events::Event;

/// Represents a value contained in a telemetry data point.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DataValue {
    #[serde(rename = "float")]
    Float(f32),
    #[serde(rename = "boolean")]
    Boolean(bool),
    #[serde(rename = "integer")]
    Integer(i32),
    #[serde(rename = "text")]
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
/// TODO: include datetime of the telemetry packet? create a ADR for it
#[derive(Debug, Serialize)]
pub struct SpacecraftTelemetry {
    pub simulation_id: String,
    pub spacecraft_id: String,
    pub mode: Mode,
    pub components: Vec<ComponentTelemetry>,
    pub events: Vec<Event>,
}
