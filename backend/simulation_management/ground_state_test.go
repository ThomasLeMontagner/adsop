package main

import (
	"encoding/json"
	"testing"
)

func TestGroundStateSerializesEmptyManagedEventsAsArray(t *testing.T) {
	groundState := GroundState{
		ManagedEvents: NewEventStore().GetEvents(),
	}

	data, err := json.Marshal(groundState)
	if err != nil {
		t.Fatalf("marshal ground state: %v", err)
	}

	var payload map[string]json.RawMessage
	if err := json.Unmarshal(data, &payload); err != nil {
		t.Fatalf("unmarshal ground state: %v", err)
	}

	if got := string(payload["managed_events"]); got != "[]" {
		t.Fatalf("managed_events = %s, want []", got)
	}
}
