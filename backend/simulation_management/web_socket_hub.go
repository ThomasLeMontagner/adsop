package main

import (
	"context"
	"encoding/json"
	"sync"

	"github.com/coder/websocket"
)

type WebSocketHub struct {
	mu      sync.RWMutex
	clients map[*websocket.Conn]struct{}
}

func NewWebSocketHub() *WebSocketHub {
	return &WebSocketHub{
		clients: make(map[*websocket.Conn]struct{}),
	}
}

func (hub *WebSocketHub) Add(connection *websocket.Conn) {
	hub.mu.Lock()
	defer hub.mu.Unlock()

	hub.clients[connection] = struct{}{}
}

func (hub *WebSocketHub) Remove(connection *websocket.Conn) {
	hub.mu.Lock()
	defer hub.mu.Unlock()

	delete(hub.clients, connection)
}

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