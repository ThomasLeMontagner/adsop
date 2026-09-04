package main

import (
	"encoding/json"
	"time"
)

type EventFromTelemetry struct {
	ID			uint32			`json:"id"`
	Timestamp 	time.Time       `json:"timestamp"`
	Source    	string          `json:"source"`
	EventType 	json.RawMessage `json:"event_type"`
	Severity  	string          `json:"severity"`
	Message   	string          `json:"message"`
}
