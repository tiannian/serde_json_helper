// Pretty formatter for JSON

use crate::SerdeConfig;

/// Pretty formatter for JSON serialization
pub struct PrettyFormatter<'a> {
    /// The underlying serde_json pretty formatter
    pub formatter: serde_json::ser::PrettyFormatter<'a>,
    /// Configuration for the formatter
    pub config: SerdeConfig,
}
