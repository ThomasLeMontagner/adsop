package main

import (
	"encoding/json"
	"net/http"

	"github.com/coder/websocket"
)

func handleCreateSimulation(simulationStore *SimulationStore) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		var simulationRequest CreateSimulationRequest

		if err := json.NewDecoder(request.Body).Decode(
			&simulationRequest,
		); err != nil {
			writeJSON(writer, http.StatusBadRequest, map[string]string{"error": "invalid JSON request"})
			return
		}

		if simulationRequest.SpacecraftID == "" {
			writeJSON(writer, http.StatusBadRequest, map[string]string{"error": "spacecraft_id is required"})
			return
		}

		simulation := simulationStore.Create()

		if err := startSimulator(
			simulation.ID,
			simulationRequest.SpacecraftID,
		); err != nil {
			writeJSON(writer, http.StatusBadGateway, map[string]string{"error": err.Error()})
			return
		}

		simulation.Status = "running"
		simulationStore.Update(simulation)

		writeJSON(writer, http.StatusCreated, simulation)
	}
}

func handleGetSimulation(simulationStore *SimulationStore) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		id := request.PathValue("id")

		simulation, found := simulationStore.Get(id)

		if !found {
			writeJSON(writer, http.StatusNotFound, map[string]string{"error": "simulation not found"})
			return
		}

		writeJSON(writer, http.StatusOK, simulation)
	}
}

func handleTelemetryIngest(telemetryStore *TelemetryStore, eventStore *EventStore, webSocketHub *WebSocketHub) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		var telemetry SpacecraftTelemetry

		if err := json.NewDecoder(request.Body).Decode(
			&telemetry,
		); err != nil {
			writeJSON(writer, http.StatusBadRequest, map[string]string{"error": "invalid telemetry"})
			return
		}

		telemetryStore.Update(telemetry)
		eventStore.Update(telemetry.Events)
		groundState := GroundState{
			SpacecraftTelemetry: telemetry,
			ManagedEvents:       eventStore.GetEvents(),
		}
		webSocketHub.Broadcast(groundState)
		writeJSON(writer, http.StatusAccepted, map[string]string{"status": "accepted"})
	}

	// todo: store the events until user acknowledgement
}

func handleGetTelemetry(telemetryStore *TelemetryStore) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		id := request.PathValue("id")

		telemetry, found := telemetryStore.Get(id)

		if !found {
			writeJSON(writer, http.StatusNotFound, map[string]string{"error": "no telemetry available"})
			return
		}

		writeJSON(writer, http.StatusOK, telemetry)
	}
}

func handleGetWebSocket(webSocketHub *WebSocketHub) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		connection, err := websocket.Accept(
			writer,
			request,
			&websocket.AcceptOptions{
				OriginPatterns: []string{"localhost:*"},
			},
		)

		if err != nil {
			return
		}

		webSocketHub.Add(connection)

		defer func() {
			webSocketHub.Remove(connection)
			connection.Close(websocket.StatusNormalClosure, "")
		}()

		for {
			_, _, err := connection.Read(request.Context())
			if err != nil {
				break
			}
		}
	}
}
