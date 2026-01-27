// Compact formatter for JSON

use crate::SerdeConfig;

/// Compact formatter for JSON serialization
pub struct CompactFormatter {
    /// The underlying serde_json compact formatter
    pub formatter: serde_json::ser::CompactFormatter,
    /// Configuration for the formatter
    pub config: SerdeConfig,
}
