package main

import (
	"log"
	"net/http"
	"time"
)

func main() {
	simulationStore := NewSimulationStore()
	telemetryStore := NewTelemetryStore()
	eventsStore := NewEventStore()
	webSocketHub := NewWebSocketHub()

	serverMux := http.NewServeMux()

	serverMux.HandleFunc(
		"GET /health",
		func(writer http.ResponseWriter, request *http.Request) {
			writeJSON(
				writer,
				http.StatusOK,
				map[string]string{"status": "ok"},
			)
		},
	)

	serverMux.HandleFunc(
		"POST /simulations",
		handleCreateSimulation(simulationStore),
	)

	serverMux.HandleFunc(
		"GET /simulations/{id}",
		handleGetSimulation(simulationStore),
	)

	serverMux.HandleFunc(
		"POST /internal/telemetry",
		handleTelemetryIngest(telemetryStore, eventsStore, webSocketHub),
	)

	serverMux.HandleFunc(
		"GET /simulations/{id}/telemetry",
		handleGetTelemetry(telemetryStore),
	)
	serverMux.HandleFunc(
		"GET /ws",
		handleGetWebSocket(webSocketHub),
	)

	server := &http.Server{
		Addr:              ":8080",
		Handler:           corsMiddleware(serverMux),
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
