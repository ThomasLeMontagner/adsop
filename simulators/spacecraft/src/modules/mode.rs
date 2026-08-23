use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum Mode {
    Nominal,
    Degraded,
    Safe,
}
