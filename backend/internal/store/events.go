package store

import (
	"errors"
	"sync"

	"github.com/ThomasLeMontagner/adsop/backend/internal/domain"
)

// EventStore retains unique events received by the ground segment.
type EventStore struct {
	mu     sync.RWMutex
	events []domain.ManagedEvent
}

// NewEventStore creates an empty in-memory event store.
func NewEventStore() *EventStore {
	return &EventStore{
		events: make([]domain.ManagedEvent, 0),
	}
}

// AddEvent stores an event unless its identifier is already present.
func (eventStore *EventStore) AddEvent(event domain.EventFromTelemetry) {
	eventStore.mu.Lock()
	defer eventStore.mu.Unlock()

	for _, managedEvent := range eventStore.events {
		if managedEvent.Event.ID == event.ID {
			return
		}
	}

	var newEvent = domain.ManagedEvent{
		Event:        event,
		Acknowledged: false,
	}
	eventStore.events = append(eventStore.events, newEvent)
}

// GetEvents returns a copy of all managed events in ingestion order.
func (eventStore *EventStore) GetEvents() []domain.ManagedEvent {
	eventStore.mu.RLock()
	defer eventStore.mu.RUnlock()

	events := make([]domain.ManagedEvent, len(eventStore.events))
	copy(events, eventStore.events)

	return events
}

// Update adds each event from a telemetry packet to the store.
func (eventStore *EventStore) Update(events []domain.EventFromTelemetry) {
	for _, event := range events {
		eventStore.AddEvent(event)
	}
}

// ErrEventNotFound indicates that an event does not exist in the store.
var ErrEventNotFound = errors.New("event not found")

// AcknowledgeEvent marks an event as acknowledged.
func (store *EventStore) AcknowledgeEvent(eventID uint32) error {
	store.mu.Lock()
	defer store.mu.Unlock()

	for index := range store.events {
		if store.events[index].Event.ID == eventID {
			store.events[index].Acknowledged = true
			return nil
		}
	}

	return ErrEventNotFound
}
