package api

import (
	"github.com/ThomasLeMontagner/adsop/backend/internal/realtime"
	"github.com/ThomasLeMontagner/adsop/backend/internal/store"
)

type Server struct {
	simulationStore *store.SimulationStore
	telemetryStore  *store.TelemetryStore
	eventStore      *store.EventStore
	webSocketHub    *realtime.WebSocketHub
}

func NewServer(
	simulationStore *store.SimulationStore,
	telemetryStore *store.TelemetryStore,
	eventStore *store.EventStore,
	webSocketHub *realtime.WebSocketHub,
) *Server {
	return &Server{
		simulationStore: simulationStore,
		telemetryStore:  telemetryStore,
		eventStore:      eventStore,
		webSocketHub:    webSocketHub,
	}
}
