package domain

import (
	"encoding/json"
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
