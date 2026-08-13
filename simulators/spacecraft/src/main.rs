pub mod modules;

use modules::telemetry::SpacecraftTelemetry;

use axum::{
    routing::{get, post}, Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tokio::time;
use crate::modules::mode::Mode;
use crate::modules::systems::{Component, PowerSystem};

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
        let mut power_system = PowerSystem{
            name: String::from("Power System"),
            battery_voltage: 28.0,
            temperature: 15.0 ,
        };

        let mut interval = time::interval(Duration::from_millis(interval_ms));
        loop {
            interval.tick().await;

            power_system.update();
            let power_system_telemetry = power_system.produce_data();
            let telemetry = SpacecraftTelemetry {
                simulation_id: simulation_id.clone(),
                spacecraft_id: spacecraft_id.clone(),
                mode: Mode::Nominal,
                components: vec![power_system_telemetry],
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
