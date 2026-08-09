package main

import "sync"

type TelemetryStore struct {
	mu     sync.RWMutex
	latest map[string]Telemetry
}

func NewTelemetryStore() *TelemetryStore {
	return &TelemetryStore{
		latest: make(map[string]Telemetry),
	}
}

func (store *TelemetryStore) Update(telemetry Telemetry) {
	store.mu.Lock()
	defer store.mu.Unlock()

	store.latest[telemetry.SimulationID] = telemetry
}

func (store *TelemetryStore) Get(simulationID string) (Telemetry, bool) {
	store.mu.RLock()
	defer store.mu.RUnlock()

	telemetry, found := store.latest[simulationID]

	return telemetry, found
}