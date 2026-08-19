use crate::modules::anomalies::Anomaly;
use crate::modules::telemetry::ComponentTelemetry;

/// Defines the behavior shared by spacecraft components.
pub trait Component {
    /// Updates the internal state.
    fn update(&mut self, dt_seconds: f32);

    /// Produces and reports state.
    fn produce_telemetry(&self) -> ComponentTelemetry;

    /// Checks the health and determines whether something is wrong.
    fn check_health(&self) -> Vec<Anomaly>;
}
