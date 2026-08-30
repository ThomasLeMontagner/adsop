use crate::modules::mode::Mode;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Identifies a condition reported by a spacecraft component.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentEvent {
    BatteryLowDetected,
    BatteryLowCleared,
    BatteryCriticalDetected,
    BatteryCriticalCleared,
    HighTemperatureDetected,
    HighTemperatureCleared,
}

impl ComponentEvent {
    /// Returns the severity associated with the component event.
    pub fn severity(&self) -> Severity {
        match self {
            ComponentEvent::BatteryLowDetected => Severity::Warning,
            ComponentEvent::BatteryCriticalDetected => Severity::Critical,
            ComponentEvent::HighTemperatureDetected => Severity::Warning,

            ComponentEvent::BatteryLowCleared
            | ComponentEvent::BatteryCriticalCleared
            | ComponentEvent::HighTemperatureCleared => Severity::Information,
        }
    }

    /// Returns a human-readable message describing the component event.
    pub fn message(&self) -> &'static str {
        match self {
            ComponentEvent::BatteryLowDetected => "Battery level is low",
            ComponentEvent::BatteryLowCleared => "Battery level returned to nominal",
            ComponentEvent::BatteryCriticalDetected => "Battery level is critical",
            ComponentEvent::BatteryCriticalCleared => "Battery is no longer critical",
            ComponentEvent::HighTemperatureDetected => "Battery temperature is too high",
            ComponentEvent::HighTemperatureCleared => "Battery temperature returned to normal",
        }
    }
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

/// Describes the domain-specific payload associated with an event.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Component(ComponentEvent),
    ModeChange(ModeChangeEvent),
}

/// Indicates the severity of an event.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// An informational event that does not indicate a fault.
    Information,
    /// An event that indicates a condition requiring attention.
    Warning,
    /// An event that indicates a condition requiring immediate attention.
    Critical,
}

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
    event: Event,
    first_sent_at: Option<DateTime<Utc>>,
    last_sent_at: Option<DateTime<Utc>>,
    retry_count: u32,
}

/// Manages spacecraft events and their transmission state.
#[derive(Default)]
pub struct EventsManager {
    pending_events: Vec<PendingEvent>,
    event_count: u32,
}

impl EventsManager {
    /// Creates an event manager with no pending events.
    pub fn new() -> Self {
        Self::default()
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
        };

        self.pending_events.push(pending_event);
    }

    /// Returns the events that are eligible for transmission.
    ///
    /// Events remain eligible until their delivery is confirmed.
    pub fn get_events_to_transmit(&self) -> Vec<Event> {
        // TODO: Prioritize events using `retry_count` and `last_sent_at`.
        self.pending_events
            .iter()
            .map(|pending_event| pending_event.event.clone())
            .collect()
    }

    /// Records a transmission attempt for the events with the supplied identifiers.
    ///
    /// The first attempt initializes the first-send timestamp. Later attempts increment the retry
    /// count. Every attempt updates the most recent send timestamp. Unknown identifiers are ignored.
    pub fn record_transmissions(&mut self, event_ids: &[u32], timestamp: DateTime<Utc>) {
        self.pending_events
            .iter_mut()
            .filter(|pending_event| event_ids.contains(&pending_event.event.id))
            .for_each(|pending_event| Self::update_transmission(pending_event, timestamp));
    }

    /// Updates the transmission metadata for a pending event.
    fn update_transmission(pending_event: &mut PendingEvent, timestamp: DateTime<Utc>) {
        if pending_event.first_sent_at.is_none() {
            pending_event.first_sent_at = Some(timestamp);
        } else {
            pending_event.retry_count += 1;
        }
        pending_event.last_sent_at = Some(timestamp);
    }

    /// Removes delivered events from the pending queue.
    ///
    /// Unknown identifiers are ignored.
    pub fn confirm_deliveries(&mut self, event_ids: &[u32]) {
        self.pending_events
            .retain(|pending_event| !event_ids.contains(&pending_event.event.id));
    }
}
