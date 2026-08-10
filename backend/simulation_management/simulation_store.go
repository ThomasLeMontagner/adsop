package main

import (
	"sync"
	"time"
)

type SimulationStore struct {
	mu          sync.RWMutex
	simulations map[string]Simulation
}

func NewSimulationStore() *SimulationStore {
	return &SimulationStore{
		simulations: make(map[string]Simulation),
	}
}

func (store *SimulationStore) Create() Simulation {
	store.mu.Lock()
	defer store.mu.Unlock()

	id := time.Now().UTC().Format("20060102T150405.000000000")

	simulation := Simulation{
		ID:        id,
		Status:    "created",
		CreatedAt: time.Now().UTC(),
	}

	store.simulations[id] = simulation

	return simulation
}

func (store *SimulationStore) Get(id string) (Simulation, bool) {
	store.mu.RLock()
	defer store.mu.RUnlock()

	simulation, found := store.simulations[id]
	return simulation, found
}

func (store *SimulationStore) Update(simulation Simulation) {
	store.mu.Lock()
	defer store.mu.Unlock()

	store.simulations[simulation.ID] = simulation
}
