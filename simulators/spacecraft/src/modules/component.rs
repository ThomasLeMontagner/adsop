use crate::modules::anomalies::Anomaly;
use crate::modules::telemetry::ComponentTelemetry;

/// Defines the behavior shared by spacecraft components.
pub trait Component {
    /// Produces telemetry.
    fn produce_telemetry(&self) -> ComponentTelemetry;

    /// Update the internal state.

    fn update(&mut self, dt_seconds: f32);

    /// Check the health.
    fn check_health(&self) -> Vec<Anomaly>;
}
