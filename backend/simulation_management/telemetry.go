package main

import "time"

// Mode identifies the spacecraft's current operating mode.
type Mode string

const (
	ModeNominal  Mode = "Nominal"
	ModeDegraded Mode = "Degraded"
	ModeSafe     Mode = "Safe"
)

// DataValue contains a typed telemetry measurement value.
type DataValue struct {
	Type  string `json:"type"`
	Value any    `json:"value"`
}

// Data is a named telemetry measurement.
type Data struct {
	Name  string    `json:"name"`
	Value DataValue `json:"value"`
}

// ComponentTelemetry groups measurements produced by one spacecraft component.
type ComponentTelemetry struct {
	Name string `json:"component_name"`
	Data []Data `json:"data"`
}

// SpacecraftTelemetry is a timestamped spacecraft state received by the backend.
type SpacecraftTelemetry struct {
	SimulationID string               `json:"simulation_id"`
	SpacecraftID string               `json:"spacecraft_id"`
	Mode         Mode                 `json:"mode"`
	Components   []ComponentTelemetry `json:"components"`
	Events       []EventFromTelemetry `json:"events"`
	Timestamp    time.Time            `json:"timestamp"`
}
