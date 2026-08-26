use crate::modules::mode::Mode;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Identifies a condition reported by a spacecraft component.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentEvent {
    LowBatteryVoltage,
    HighTemperature,
}

/// Records a transition from one spacecraft operating mode to another.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

impl ModeChangeEvent {
    /// Creates a mode-change payload with the previous and new operating modes.
    pub fn new(from: Mode, to: Mode) -> Self {
        Self { from, to }
    }
}

/// Describes the domain-specific payload associated with an event.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Component(ComponentEvent),
    ModeChange(ModeChangeEvent),
}

/// Indicates the severity of an event.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// An informational event that does not indicate a fault.
    Information,
    /// An event that indicates a condition requiring attention.
    Warning,
}

/// Represents a timestamped event emitted by the spacecraft or a component.
#[derive(Debug, Serialize, PartialEq)]
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
    event_count: u32,
}

impl EventsManager {
    /// Creates an event manager with no pending events.
    pub fn new() -> Self {
        Self {
            pending_events: Vec::new(),
            event_count: 0,
        }
    }

    /// Creates an event with the next identifier and adds it to the pending queue.
    pub fn add_event(
        &mut self,
        source: String,
        event_type: EventType,
        severity: Severity,
        message: String,
    ) {
        self.event_count += 1;
        let event = Event::new(self.event_count, source, event_type, severity, message);
        self.add_pending_event(event);
    }

    /// Adds an event to the pending queue with no transmission attempts recorded.
    fn add_pending_event(&mut self, event: Event) {
        let pending_event = PendingEvent {
            event,
            first_sent_at: None,
            last_sent_at: None,
            retry_count: 0,
            delivery_confirmed: false,
        };

        self.pending_events.push(pending_event);
    }

    /// Returns the events that are eligible for transmission.
    ///
    /// Events remain eligible until their delivery is confirmed.
    pub fn get_events_to_transmit(&self) -> Vec<&Event> {
        self.pending_events
            .iter()
            .filter(|pending_event| Self::should_be_transmitted(pending_event))
            .map(|pending_event| &pending_event.event)
            .collect()
    }

    /// Returns whether a pending event is currently eligible for transmission.
    fn should_be_transmitted(event: &PendingEvent) -> bool {
        // TODO: Prioritize events using `retry_count` and `last_sent_at`.
        !event.delivery_confirmed
    }

    /// Records a transmission attempt for the event with the supplied identifier.
    ///
    /// The first attempt initializes the first-send timestamp. Later attempts increment the retry
    /// count. Every attempt updates the most recent send timestamp. An unknown identifier is
    /// ignored.
    pub fn confirm_transmission(&mut self, event_id: u32, timestamp: DateTime<Utc>) {
        self.pending_events
            .iter_mut()
            .filter(|pending_event| pending_event.event.id == event_id)
            .for_each(|pending_event| Self::update_transmission(pending_event, &timestamp));
    }

    /// Updates the transmission metadata for a pending event.
    fn update_transmission(pending_event: &mut PendingEvent, timestamp: &DateTime<Utc>) {
        if pending_event.first_sent_at.is_none() {
            pending_event.first_sent_at = Some(timestamp.clone());
        } else {
            pending_event.retry_count += 1;
        }
        pending_event.last_sent_at = Some(timestamp.clone());
    }

    /// Marks the event with the supplied identifier as delivered.
    ///
    /// An unknown identifier is ignored.
    pub fn confirm_delivery(&mut self, event_id: u32) {
        self.pending_events
            .iter_mut()
            .filter(|pending_event| pending_event.event.id == event_id)
            .for_each(|pending_event| pending_event.delivery_confirmed = true)
    }
}
