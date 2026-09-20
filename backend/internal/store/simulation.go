package store

import (
	"sync"
	"time"

	"github.com/ThomasLeMontagner/adsop/backend/internal/domain"
)

// SimulationStore holds simulations in memory.
type SimulationStore struct {
	mu          sync.RWMutex
	simulations map[string]domain.Simulation
}

// NewSimulationStore creates an empty simulation store.
func NewSimulationStore() *SimulationStore {
	return &SimulationStore{
		simulations: make(map[string]domain.Simulation),
	}
}

// Create adds and returns a new simulation.
func (store *SimulationStore) Create() domain.Simulation {
	store.mu.Lock()
	defer store.mu.Unlock()

	id := time.Now().UTC().Format("20060102T150405.000000000")

	simulation := domain.Simulation{
		ID:        id,
		Status:    "created",
		CreatedAt: time.Now().UTC(),
	}

	store.simulations[id] = simulation

	return simulation
}

// Get retrieves a simulation by ID.
func (store *SimulationStore) Get(id string) (domain.Simulation, bool) {
	store.mu.RLock()
	defer store.mu.RUnlock()

	simulation, found := store.simulations[id]
	return simulation, found
}

// Update stores the latest state of a simulation.
func (store *SimulationStore) Update(simulation domain.Simulation) {
	store.mu.Lock()
	defer store.mu.Unlock()

	store.simulations[simulation.ID] = simulation
}
