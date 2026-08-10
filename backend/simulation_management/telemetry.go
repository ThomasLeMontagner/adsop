package main

type Telemetry struct {
	SimulationID   string  `json:"simulation_id"`
	SpacecraftID   string  `json:"spacecraft_id"`
	BatteryVoltage float64 `json:"battery_voltage"`
}
