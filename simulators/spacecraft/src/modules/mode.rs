use serde::Serialize;

/// Represents the current operating mode of the spacecraft.
#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum Mode {
    Nominal,
    Degraded,
    Safe,
}
