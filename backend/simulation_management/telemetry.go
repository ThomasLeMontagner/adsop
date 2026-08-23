package main

import "time"

type Mode string

const (
	ModeNominal  Mode = "Nominal"
	ModeDegraded Mode = "Degraded"
	ModeSafe     Mode = "Sage"
)

type DataValue struct {
	Type  string `json:"type"`
	Value any    `json:"value"`
}

type Data struct {
	Name  string    `json:"name"`
	Value DataValue `json:"value"`
}

type ComponentTelemetry struct {
	Name string `json:"component_name"`
	Data []Data `json:"data"`
}

type SpacecraftTelemetry struct {
	SimulationID string               `json:"simulation_id"`
	SpacecraftID string               `json:"spacecraft_id"`
	Mode         Mode                 `json:"mode"`
	Components   []ComponentTelemetry `json:"components"`
	Events       []EventFromTelemetry `json:"events"`
	Timestamp    time.Time            `json:"timestamp"`
}
