pub mod modules;

use modules::telemetry::SpacecraftTelemetry;

use crate::modules::spacecraft::Spacecraft;
use axum::{
    Json, Router,
    routing::{get, post},
};
use modules::component::Component;
use modules::power_system::PowerSystem;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tokio::time;

#[derive(Debug, Deserialize)]
struct StartSimulationRequest {
    simulation_id: String,
    spacecraft_id: String,
    telemetry_interval_ms: u64,
}

#[derive(Debug, Serialize)]
struct StartSimulationResponse {
    simulation_id: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/simulations/start", post(start_simulation));

    let address = SocketAddr::from(([127, 0, 0, 1], 8090));

    println!("Spacecraft simulator listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind simulator address");

    axum::serve(listener, app)
        .await
        .expect("simulator server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn start_simulation(
    Json(request): Json<StartSimulationRequest>,
) -> Json<StartSimulationResponse> {
    let simulation_id = request.simulation_id.clone();
    let spacecraft_id = request.spacecraft_id.clone();
    let interval_ms = request.telemetry_interval_ms.clone();
    let client = reqwest::Client::new();

    tokio::spawn(async move {
        let mut spacecraft = Spacecraft::new(
            spacecraft_id.clone(),
            PowerSystem {
                name: String::from("Power System"),
                battery_capacity_wh: 1500.0,
                battery_energy_wh: 1400.0,
                consumed_power_w: 350.0,
                solar_array_generating_power: false,
                battery_temperature: 15.0,
            },
        );
        let mut interval = time::interval(Duration::from_millis(interval_ms));
        loop {
            interval.tick().await;

            let dt_seconds = interval_ms as f32 / 1000.0;
            spacecraft.power_system.update(dt_seconds);
            spacecraft.evaluate_autonomous_rules();
            let power_system_telemetry = spacecraft.power_system.produce_telemetry();
            let telemetry = SpacecraftTelemetry {
                simulation_id: simulation_id.clone(),
                spacecraft_id: spacecraft_id.clone(),
                mode: spacecraft.mode(),
                components: vec![power_system_telemetry],
                events: vec![],
            };

            let result = client
                .post("http://localhost:8080/internal/telemetry")
                .json(&telemetry)
                .send()
                .await;

            match result {
                Ok(response) => {
                    if !response.status().is_success() {
                        eprintln!("Backend rejected telemetry: {}", response.status());
                    }
                }
                Err(error) => {
                    eprintln!("Failed to send telemetry: {}", error);
                }
            }
            println!(
                "{}",
                serde_json::to_string(&telemetry).expect("failed to serialize telemetry")
            );
        }
    });

    Json(StartSimulationResponse {
        simulation_id: request.simulation_id,
        status: "running".to_string(),
    })
}
