/// Describes an anomalous condition detected in a spacecraft component.
#[derive(Debug, PartialEq, Eq)]
pub enum Anomaly {
    BatteryLow,
    BatteryCritical,
    BatteryOverheating,
}
