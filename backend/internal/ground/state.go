package ground

import "github.com/ThomasLeMontagner/adsop/backend/internal/domain"

// GroundState is the telemetry and managed-event snapshot sent to frontend clients.
type GroundState struct {
	SpacecraftTelemetry domain.SpacecraftTelemetry `json:"spacecraft_telemetry"`
	ManagedEvents       []domain.ManagedEvent      `json:"managed_events"`
}
