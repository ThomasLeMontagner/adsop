package main

import (
	"encoding/json"
	"sync"
	"time"
)

type EventFromTelemetry struct {
	ID        uint32          `json:"id"`
	Timestamp time.Time       `json:"timestamp"`
	Source    string          `json:"source"`
	EventType json.RawMessage `json:"event_type"`
	Severity  string          `json:"severity"`
	Message   string          `json:"message"`
}

type ManagedEvent struct {
	Event        EventFromTelemetry `json:"event"`
	Acknowledged bool               `json:"acknowledged"`
}

type EventStore struct {
	mu     sync.RWMutex
	events []ManagedEvent
}

func NewEventStore() *EventStore {
	return &EventStore{
		events: make([]ManagedEvent, 0),
	}
}

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

func (eventStore *EventStore) GetEvents() []ManagedEvent {
	eventStore.mu.RLock()
	defer eventStore.mu.RUnlock()

	events := make([]ManagedEvent, len(eventStore.events))
	copy(events, eventStore.events)

	return events
}

func (eventStore *EventStore) Update(events []EventFromTelemetry) {
	for _, event := range events {
		eventStore.AddEvent(event)
	}
}
