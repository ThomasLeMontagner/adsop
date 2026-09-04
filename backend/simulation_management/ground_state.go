package main

// GroundState is the telemetry and managed-event snapshot sent to frontend clients.
type GroundState struct {
	SpacecraftTelemetry SpacecraftTelemetry `json:"spacecraft_telemetry"`
	ManagedEvents       []ManagedEvent      `json:"managed_events"`
}
