/// Represents the current battery health state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatteryHealth {
    Nominal,
    Low,
    Critical,
}
