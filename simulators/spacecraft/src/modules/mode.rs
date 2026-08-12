use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Mode {
    Normal,
    Degraded,
    Safe,
}