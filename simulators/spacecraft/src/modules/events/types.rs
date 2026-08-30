use crate::modules::subsystems::{ComponentEvent, ModeChangeEvent};
use serde::Serialize;

/// Describes the domain-specific payload associated with an event.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Component(ComponentEvent),
    ModeChange(ModeChangeEvent),
}
