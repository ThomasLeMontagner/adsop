package main

import (
	"encoding/json"
	"sync"
	"time"
)

// EventFromTelemetry is an event received in a spacecraft telemetry packet.
type EventFromTelemetry struct {
	ID        uint32          `json:"id"`
	Timestamp time.Time       `json:"timestamp"`
	Source    string          `json:"source"`
	EventType json.RawMessage `json:"event_type"`
	Severity  string          `json:"severity"`
	Message   string          `json:"message"`
}

// ManagedEvent adds ground-side acknowledgement state to a spacecraft event.
type ManagedEvent struct {
	Event        EventFromTelemetry `json:"event"`
	Acknowledged bool               `json:"acknowledged"`
}

// EventStore retains unique events received by the ground segment.
type EventStore struct {
	mu     sync.RWMutex
	events []ManagedEvent
}

// NewEventStore creates an empty in-memory event store.
func NewEventStore() *EventStore {
	return &EventStore{
		events: make([]ManagedEvent, 0),
	}
}

// AddEvent stores an event unless its identifier is already present.
func (eventStore *EventStore) AddEvent(event EventFromTelemetry) {
	eventStore.mu.Lock()
	defer eventStore.mu.Unlock()

	for _, managedEvent := range eventStore.events {
		if managedEvent.Event.ID == event.ID {
			return
		}
	}

	var newEvent = ManagedEvent{
		Event:        event,
		Acknowledged: false,
	}
	eventStore.events = append(eventStore.events, newEvent)
}

// GetEvents returns a copy of all managed events in ingestion order.
func (eventStore *EventStore) GetEvents() []ManagedEvent {
	eventStore.mu.RLock()
	defer eventStore.mu.RUnlock()

	events := make([]ManagedEvent, len(eventStore.events))
	copy(events, eventStore.events)

	return events
}

// Update adds each event from a telemetry packet to the store.
func (eventStore *EventStore) Update(events []EventFromTelemetry) {
	for _, event := range events {
		eventStore.AddEvent(event)
	}
}
