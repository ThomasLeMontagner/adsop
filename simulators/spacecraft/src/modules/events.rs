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

/// Tracks the transmission state of an event awaiting delivery confirmation.
pub struct PendingEvent {
    event: Event,
    first_sent_at: Option<DateTime<Utc>>,
    last_sent_at: Option<DateTime<Utc>>,
    retry_count: u32,
    delivery_confirmed: bool,
}

/// Manages spacecraft events and their transmission state.
pub struct EventsManager {
    pending_events: Vec<PendingEvent>,
}

impl EventsManager {
    /// Creates an empty event manager.
    pub fn new() -> Self {
        Self {
            pending_events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        let pending_event = PendingEvent{
            event,
            first_sent_at: None,
            last_sent_at: None,
            retry_count: 0,
            delivery_confirmed: false,
        };

        self.pending_events.push(pending_event);
    }

    /// Returns the events that still need delivery confirmation.
    /// todo: prioritize events to sent based on retry_count and last_sent_at
    pub fn get_events_to_transmit(&self) -> Vec<&Event> {
        self.pending_events
            .iter()
            .filter(|pending_event| !pending_event.delivery_confirmed)
            .map(|pending_event| &pending_event.event)
            .collect()
    }


    // update pending event with trasmission date
    pub fn confirm_transmission(&mut self, event: &Event, timestamp: DateTime<Utc>) {}

    // update confirmed delivery status for an event.
    pub fn confirm_delivery(&mut self, event: &Event) {}
}
