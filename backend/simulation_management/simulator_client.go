package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
)

func startSimulator(
	simulationID string,
	spacecraftID string,
) error {
	requestBody := StartSimulationRequest{
		SimulationID:        simulationID,
		SpacecraftID:        spacecraftID,
		TelemetryIntervalMS: 1000,
	}

	body, err := json.Marshal(requestBody)
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