use crate::modules::subsystems::component_events::ComponentEvent;
use crate::modules::telemetry::ComponentTelemetry;

/// Defines the behavior shared by spacecraft components.
pub trait Component {
    /// Updates the internal state.
    fn update(&mut self, dt_seconds: f32);

    /// Produces and reports state.
    fn produce_telemetry(&self) -> ComponentTelemetry;

    /// Derives component events based on anomalies and health state.
    fn evaluate_health(&mut self) -> Vec<ComponentEvent>;
}
