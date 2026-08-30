use crate::modules::events::Severity;
use crate::modules::events::types::EventType;
use crate::modules::events::{Event, PendingEvent};
use chrono::{DateTime, Utc};

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
            .filter(|pending_event| event_ids.contains(&pending_event.event.id()))
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
            .retain(|pending_event| !event_ids.contains(&pending_event.event.id()));
    }
}
