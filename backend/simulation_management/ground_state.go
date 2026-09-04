package main

type GroundState struct {
	SpacecraftTelemetry SpacecraftTelemetry `json:"spacecraft_telemetry"`
	ManagedEvents       []ManagedEvent      `json:"managed_events"`
}
