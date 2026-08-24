use crate::modules::mode::Mode;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Identifies an event raised by a spacecraft component.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentEvent {
    LowBatteryVoltage,
    HighTemperature,
}

/// Describes a transition between spacecraft operating modes.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

impl ModeChangeEvent {
    /// Creates an event for a transition between two operating modes.
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }
}

/// Identifies the payload associated with an event.
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

/// Represents a timestamped event emitted by the spacecraft or a component.
#[derive(Debug, Serialize)]
pub struct Event {
    timestamp: DateTime<Utc>,
    source: String,
    event_type: EventType,
    severity: Severity,
    message: String,
}

impl Event {
    /// Creates an event stamped with the current UTC time.
    pub fn new(source: String, event_type: EventType, severity: Severity, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            source,
            event_type,
            severity,
            message,
        }
    }
}
