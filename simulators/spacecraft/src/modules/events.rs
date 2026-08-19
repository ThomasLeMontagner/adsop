use crate::modules::mode::Mode;
use serde::Serialize;

#[derive(Debug, Serialize)]
enum ComponentEvent {
    LowBatteryVoltage,
    HighTemperature,
}

#[derive(Debug, Serialize)]
pub struct ModeChangeEvent {
    from: Mode,
    to: Mode,
}

#[derive(Debug, Serialize)]
pub enum Event {
    ComponentEvent,
    ModeChangeEvent,
}
