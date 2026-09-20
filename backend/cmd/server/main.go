package main

import (
	"log"
	"net/http"
	"time"

	"github.com/ThomasLeMontagner/adsop/backend/internal/api"
	"github.com/ThomasLeMontagner/adsop/backend/internal/realtime"
	"github.com/ThomasLeMontagner/adsop/backend/internal/store"
)

func main() {

	simulationStore := store.NewSimulationStore()
	telemetryStore := store.NewTelemetryStore()
	eventsStore := store.NewEventStore()
	webSocketHub := realtime.NewWebSocketHub()

	apiServer := api.NewServer(simulationStore, telemetryStore, eventsStore, webSocketHub)

	serverMux := http.NewServeMux()

	serverMux.HandleFunc(
		"GET /health",
		api.CheckHealth(),
	)

	serverMux.HandleFunc(
		"POST /simulations",
		api.HandleCreateSimulation(simulationStore),
	)

	serverMux.HandleFunc(
		"GET /simulations/{id}",
		api.HandleGetSimulation(simulationStore),
	)

	serverMux.HandleFunc(
		"POST /internal/telemetry",
		api.HandleTelemetryIngest(apiServer),
	)

	serverMux.HandleFunc(
		"GET /simulations/{id}/telemetry",
		api.HandleGetTelemetry(telemetryStore),
	)

	serverMux.HandleFunc(
		"GET /ws",
		api.HandleGetWebSocket(webSocketHub),
	)

	serverMux.HandleFunc(
		"POST /events/{eventId}/acknowledge",
		api.AcknowledgeEventHandler(eventsStore),
	)

	server := &http.Server{
		Addr:              ":8080",
		Handler:           api.CorsMiddleware(serverMux),
		ReadHeaderTimeout: 5 * time.Second,
	}

	log.Println(
		"ADSOP backend listening on http://localhost:8080",
	)

	if err := server.ListenAndServe(); err != nil &&
		err != http.ErrServerClosed {
		log.Fatal(err)
	}
}
