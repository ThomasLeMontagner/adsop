use serde::Serialize;

/// Indicates the severity of an event.
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// An informational event that does not indicate a fault.
    Information,
    /// An event that indicates a condition requiring attention.
    Warning,
    /// An event that indicates a condition requiring immediate attention.
    Critical,
}
