use crate::modules::mode::Mode;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentEvent {
    LowBatteryVoltage,
    HighTemperature,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

impl ModeChangeEvent {
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }
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

impl Event {
    pub fn new(source: String, event_type: EventType, severity: Severity, message: String) -> Self {
        Self {
            time: Utc::now(),
            source,
            event_type,
            severity,
            message,
        }
    }
}
