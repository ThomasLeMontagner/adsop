use crate::modules::events::severity::Severity;
use crate::modules::events::types::EventType;
use crate::modules::mode::Mode;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Represents a timestamped event emitted by the spacecraft or a component.
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Event {
    id: u32,
    timestamp: DateTime<Utc>,
    source: String,
    event_type: EventType,
    severity: Severity,
    message: String,
}

impl Event {
    /// Creates an event with the supplied identifier and the current UTC timestamp.
    pub fn new(
        id: u32,
        source: String,
        event_type: EventType,
        severity: Severity,
        message: String,
    ) -> Self {
        Self {
            id,
            timestamp: Utc::now(),
            source,
            event_type,
            severity,
            message,
        }
    }

    /// Returns the identifier used to track this event through delivery.
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Tracks the transmission state of an event awaiting delivery confirmation.
pub struct PendingEvent {
    pub event: Event,
    pub first_sent_at: Option<DateTime<Utc>>,
    pub last_sent_at: Option<DateTime<Utc>>,
    pub retry_count: u32,
}

/// Records a transition from one spacecraft operating mode to another.
#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

impl ModeChangeEvent {
    /// Returns the severity associated with the change of modes.
    pub fn severity(&self) -> Severity {
        match (self.from, self.to) {
            (Mode::Nominal, Mode::Degraded) => Severity::Warning,
            (_, Mode::Safe) => Severity::Critical,
            (Mode::Safe, _) => Severity::Information,
            (Mode::Degraded, Mode::Nominal) => Severity::Information,
            _ => Severity::Information,
        }
    }

    /// Returns a human-readable message describing the component event.
    pub fn message(&self) -> String {
        format!("Mode changed from {:?} to {:?}", self.from, self.to)
    }
}

impl ModeChangeEvent {
    /// Creates a mode-change payload with the previous and new operating modes.
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }
}
