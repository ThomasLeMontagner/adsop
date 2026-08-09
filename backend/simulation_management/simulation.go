package main

import "time"

type Simulation struct {
	ID        string    `json:"id"`
	Status    string    `json:"status"`
	CreatedAt time.Time `json:"created_at"`
}

type CreateSimulationRequest struct {
	SpacecraftID string `json:"spacecraft_id"`
}

type StartSimulationRequest struct {
	SimulationID        string `json:"simulation_id"`
	SpacecraftID        string `json:"spacecraft_id"`
	TelemetryIntervalMS int    `json:"telemetry_interval_ms"`
}