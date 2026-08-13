use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Mode {
    Nominal,
    Degraded,
    Safe,
}