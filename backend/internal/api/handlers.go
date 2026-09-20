package api

import (
	"encoding/json"
	"errors"
	"net/http"
	"strconv"

	"github.com/coder/websocket"

	"github.com/ThomasLeMontagner/adsop/backend/internal/domain"
	"github.com/ThomasLeMontagner/adsop/backend/internal/ground"
	"github.com/ThomasLeMontagner/adsop/backend/internal/realtime"
	"github.com/ThomasLeMontagner/adsop/backend/internal/simulator"
	"github.com/ThomasLeMontagner/adsop/backend/internal/store"
)

// CheckHealth returns a handler that reports the API's health status.
func CheckHealth() http.HandlerFunc {
	return func(writer http.ResponseWriter, request *http.Request) {
		writeJSON(
			writer,
			http.StatusOK,
			map[string]string{"status": "ok"},
		)
	}
}

// HandleCreateSimulation returns a handler that creates and starts a simulation.
func HandleCreateSimulation(simulationStore *store.SimulationStore) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		var simulationRequest domain.CreateSimulationRequest

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

		if err := simulator.StartSimulator(
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

// HandleGetSimulation returns a handler that retrieves a simulation by ID.
func HandleGetSimulation(simulationStore *store.SimulationStore) http.HandlerFunc {
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

// HandleTelemetryIngest returns a handler that stores and broadcasts telemetry.
func HandleTelemetryIngest(server *Server) http.HandlerFunc {
	return func(
		writer http.ResponseWriter,
		request *http.Request,
	) {
		var telemetry domain.SpacecraftTelemetry

		if err := json.NewDecoder(request.Body).Decode(
			&telemetry,
		); err != nil {
			writeJSON(writer, http.StatusBadRequest, map[string]string{"error": "invalid telemetry"})
			return
		}

		server.telemetryStore.Update(telemetry)
		server.eventStore.Update(telemetry.Events)
		groundState := ground.GroundState{
			SpacecraftTelemetry: telemetry,
			ManagedEvents:       server.eventStore.GetEvents(),
		}
		server.webSocketHub.Broadcast(groundState)
		writeJSON(writer, http.StatusAccepted, map[string]string{"status": "accepted"})
	}
}

// HandleGetTelemetry returns a handler that retrieves telemetry by spacecraft ID.
func HandleGetTelemetry(telemetryStore *store.TelemetryStore) http.HandlerFunc {
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

// HandleGetWebSocket returns a handler that manages a WebSocket connection.
func HandleGetWebSocket(webSocketHub *realtime.WebSocketHub) http.HandlerFunc {
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

// AcknowledgeEventHandler returns a handler that acknowledges an event by ID.
func AcknowledgeEventHandler(eventStore *store.EventStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		eventIDText := r.PathValue("eventId")

		eventID64, err := strconv.ParseUint(eventIDText, 10, 32)
		if err != nil {
			http.Error(w, "Invalid event ID", http.StatusBadRequest)
			return
		}

		eventID := uint32(eventID64)

		err = eventStore.AcknowledgeEvent(eventID)
		if err != nil {
			if errors.Is(err, store.ErrEventNotFound) {
				http.Error(w, "Event not found", http.StatusNotFound)
				return
			}

			http.Error(w, "Could not acknowledge event", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusNoContent)
	}
}
