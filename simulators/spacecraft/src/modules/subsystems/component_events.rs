use crate::modules::events::Severity;
use serde::Serialize;

/// Identifies a condition reported by a spacecraft component.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentEvent {
    BatteryLowDetected,
    BatteryLowCleared,
    BatteryCriticalDetected,
    BatteryCriticalCleared,
    HighTemperatureDetected,
    HighTemperatureCleared,
}

impl ComponentEvent {
    /// Returns the severity associated with the component event.
    pub fn severity(&self) -> Severity {
        match self {
            ComponentEvent::BatteryLowDetected => Severity::Warning,
            ComponentEvent::BatteryCriticalDetected => Severity::Critical,
            ComponentEvent::HighTemperatureDetected => Severity::Warning,

            ComponentEvent::BatteryLowCleared
            | ComponentEvent::BatteryCriticalCleared
            | ComponentEvent::HighTemperatureCleared => Severity::Information,
        }
    }

    /// Returns a human-readable message describing the component event.
    pub fn message(&self) -> &'static str {
        match self {
            ComponentEvent::BatteryLowDetected => "Battery level is low",
            ComponentEvent::BatteryLowCleared => "Battery level returned to nominal",
            ComponentEvent::BatteryCriticalDetected => "Battery level is critical",
            ComponentEvent::BatteryCriticalCleared => "Battery is no longer critical",
            ComponentEvent::HighTemperatureDetected => "Battery temperature is too high",
            ComponentEvent::HighTemperatureCleared => "Battery temperature returned to normal",
        }
    }
}
