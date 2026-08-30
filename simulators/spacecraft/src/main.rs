pub mod modules;

use crate::modules::spacecraft::Spacecraft;
use crate::modules::subsystems::power::PowerSystem;
use axum::{
    Json, Router,
    routing::{get, post},
};
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
    let interval_ms = request.telemetry_interval_ms;
    let client = reqwest::Client::new();

    tokio::spawn(async move {
        let mut spacecraft = Spacecraft::new(
            spacecraft_id.clone(),
            PowerSystem::new(
                String::from("Power System"),
                1500.0,
                1400.0,
                350.0,
                15.0,
                false,
            ),
        );
        let mut interval = time::interval(Duration::from_millis(interval_ms));
        loop {
            interval.tick().await;

            let dt_seconds = interval_ms as f32 / 1000.0;
            spacecraft.update(dt_seconds);

            let telemetry = spacecraft.produce_telemetry(&simulation_id);
            let event_ids = telemetry
                .events
                .iter()
                .map(|event| event.id())
                .collect::<Vec<_>>();
            spacecraft.record_event_transmissions(&event_ids);

            let result = client
                .post("http://localhost:8080/internal/telemetry")
                .json(&telemetry)
                .send()
                .await;

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        spacecraft.confirm_event_deliveries(&event_ids);
                    } else {
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
