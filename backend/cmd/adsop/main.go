package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"
)

type Simulation struct {
	ID        string    `json:"id"`
	Status    string    `json:"status"`
	CreatedAt time.Time `json:"created_at"`
}

type StartSimulationRequest struct {
	SimulationID 		string 	`json:"simulation_id"`
	SpacecraftID 		string 	`json:"spacecraft_id"`
	TelemetryIntervalMS	int		`json:"telemetry_interval_ms"`
}
type CreateSimulationRequest struct {
	SpacecraftID string `json:"spacecraft_id"`
}

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

	id := time.Now().UTC().Format("20260704T120530.000000000")

	simulation := Simulation{
		ID:        id,
		Status:    "created",
		CreatedAt: time.Now().UTC(),
	}

	store.simulations[id] = simulation
	return simulation
}

func writeJSON(writer http.ResponseWriter, status int, value any) {
	writer.Header().Set("Content-Type", "application/json")
	writer.WriteHeader(status)

	if err := json.NewEncoder(writer).Encode(value); err != nil {
		log.Printf("failed to encode response: %v", err)
	}
}

func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		writer.Header().Set(
			"Access-Control-Allow-Origin",
			"http://localhost:5173",
		)
		writer.Header().Set(
			"Access-Control-Allow-Headers",
			"Content-Type",
		)
		writer.Header().Set(
			"Access-Control-Allow-Methods",
			"GET, POST, OPTIONS",
		)

		if request.Method == http.MethodOptions {
			writer.WriteHeader(http.StatusNoContent)
			return
		}

		next.ServeHTTP(writer, request)
	})
}

func startSimulator(
	simulationID string,
	spacecraftID string,
) error {
	request_body := StartSimulationRequest{
		SimulationID: simulationID,
		SpacecraftID: spacecraftID,
		TelemetryIntervalMS: 1000,
	}

	body, err := json.Marshal(request_body)
	if err != nil {
		return fmt.Errorf("encode simulator request: %w", err)
	}

	response, err := http.Post(
		"http://localhost:8090/simulations/start",
		"application/json",
		bytes.NewReader(body),
	)

	if err != nil {
		return fmt.Errorf("call simulator: %w", err)
	}
	defer response.Body.Close()

	if response.StatusCode < 200 || response.StatusCode >= 300 {
		return fmt.Errorf(
			"simulator returned status %d",
			response.StatusCode,
		)
	}

	return nil
}

func (store *SimulationStore) Update(simulation Simulation) {
	store.mu.Lock()
	defer store.mu.Unlock()

	store.simulations[simulation.ID] = simulation
}

func main() {
	store := NewSimulationStore()
	server_mux := http.NewServeMux()

	server_mux.HandleFunc("GET /health", func(writer http.ResponseWriter, request *http.Request) {
		writeJSON(writer, http.StatusOK, map[string]string{
			"status": "ok",
		})
	})

	server_mux.HandleFunc("POST /simulations", func(writer http.ResponseWriter, request_ *http.Request) {
		var request CreateSimulationRequest

		if err := json.NewDecoder(request_.Body).Decode(&request); err != nil {
			writeJSON(writer, http.StatusBadRequest, map[string]string{
				"error": "invalid JSON request",
			})
			return
		}

		if request.SpacecraftID == "" {
			writeJSON(writer, http.StatusBadRequest, map[string]string{
				"error": "spacecraft_id is required",
			})
			return
		}

		simulation := store.Create()
		
		if err := startSimulator(simulation.ID, request.SpacecraftID); err != nil {
			writeJSON(writer, http.StatusBadGateway, map[string]string {
				"error": err.Error(),
			})
			return
		}

		simulation.Status = "running"
		store.Update(simulation)
		writeJSON(writer, http.StatusCreated, simulation)
	})

	server_mux.HandleFunc("GET /simulations/{id}", func(writer http.ResponseWriter, request *http.Request){
		id := request.PathValue("id")

		simulation, exists := store.simulations[id]

		if exists{
			writeJSON(writer, http.StatusOK, simulation)
		} else {
			writeJSON(writer, http.StatusBadRequest, map[string]string{
				"error": "spacecraft_id is incorrect",
			})
		}
		
	})

	server := &http.Server{
		Addr:              ":8080",
		Handler:           corsMiddleware(server_mux),
		ReadHeaderTimeout: 5 * time.Second,
	}

	log.Println("ADSOP backend listening on http://localhost:8080")

	if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		log.Fatal(err)
	}
}
