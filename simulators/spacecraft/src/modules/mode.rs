use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Mode {
    Nominal,
    Degraded,
    Safe,
}
