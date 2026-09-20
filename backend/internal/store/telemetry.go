package store

import (
	"sync"

	"github.com/ThomasLeMontagner/adsop/backend/internal/domain"
)

// TelemetryStore holds the latest telemetry for each simulation.
type TelemetryStore struct {
	mu     sync.RWMutex
	latest map[string]domain.SpacecraftTelemetry
}

// NewTelemetryStore creates an empty telemetry store.
func NewTelemetryStore() *TelemetryStore {
	return &TelemetryStore{
		latest: make(map[string]domain.SpacecraftTelemetry),
	}
}

// Update stores the latest telemetry for a simulation.
func (store *TelemetryStore) Update(telemetry domain.SpacecraftTelemetry) {
	store.mu.Lock()
	defer store.mu.Unlock()

	store.latest[telemetry.SimulationID] = telemetry
}

// Get retrieves the latest telemetry for a simulation.
func (store *TelemetryStore) Get(simulationID string) (domain.SpacecraftTelemetry, bool) {
	store.mu.RLock()
	defer store.mu.RUnlock()

	telemetry, found := store.latest[simulationID]

	return telemetry, found
}
