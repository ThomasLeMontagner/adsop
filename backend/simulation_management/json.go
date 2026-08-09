package main

import (
	"encoding/json"
	"log"
	"net/http"
)

func writeJSON(
	writer http.ResponseWriter,
	status int,
	value any,
) {
	writer.Header().Set("Content-Type", "application/json")
	writer.WriteHeader(status)

	if err := json.NewEncoder(writer).Encode(value); err != nil {
		log.Printf("failed to encode response: %v", err)
	}
}