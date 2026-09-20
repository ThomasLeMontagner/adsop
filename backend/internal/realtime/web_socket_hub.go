package realtime

import (
	"context"
	"encoding/json"
	"sync"

	"github.com/coder/websocket"
)

// WebSocketHub manages active WebSocket clients and broadcasts messages to them.
type WebSocketHub struct {
	mu      sync.RWMutex
	clients map[*websocket.Conn]struct{}
}

// NewWebSocketHub creates an empty WebSocket hub.
func NewWebSocketHub() *WebSocketHub {
	return &WebSocketHub{
		clients: make(map[*websocket.Conn]struct{}),
	}
}

// Add registers a WebSocket connection with the hub.
func (hub *WebSocketHub) Add(connection *websocket.Conn) {
	hub.mu.Lock()
	defer hub.mu.Unlock()

	hub.clients[connection] = struct{}{}
}

// Remove unregisters a WebSocket connection from the hub.
func (hub *WebSocketHub) Remove(connection *websocket.Conn) {
	hub.mu.Lock()
	defer hub.mu.Unlock()

	delete(hub.clients, connection)
}

// Broadcast sends a JSON-encoded value to all connected clients.
func (hub *WebSocketHub) Broadcast(value any) {
	data, err := json.Marshal(value)
	if err != nil {
		return
	}

	hub.mu.RLock()
	clients := make([]*websocket.Conn, 0, len(hub.clients))

	for connection := range hub.clients {
		clients = append(clients, connection)
	}

	hub.mu.RUnlock()

	for _, connection := range clients {
		err := connection.Write(context.Background(), websocket.MessageText, data)

		if err != nil {
			hub.Remove(connection)
			connection.Close(websocket.StatusInternalError, "write failed")
		}
	}
}
