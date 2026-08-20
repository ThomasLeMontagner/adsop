use crate::modules::mode::Mode;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ComponentEvent {
    LowBatteryVoltage,
    HighTemperature,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Component(ComponentEvent),
    ModeChange(ModeChangeEvent),
}

/// Severity of an event.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Information,
    Warning,
}

#[derive(Debug, Serialize)]
pub struct Event {
    time: DateTime<Utc>,
    source: String,
    event_type: EventType,
    severity: Severity,
    message: String,
}
