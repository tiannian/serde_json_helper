// Bytes serialization utilities

use crate::{BytesFormat, Config};
use std::io::Write;

/// Serializes bytes to JSON format based on the configuration
///
/// # Arguments
///
/// * `writer` - A mutable reference to a writer that implements `Write`
/// * `formatter` - A formatter that implements `serde_json::ser::Formatter`
/// * `config` - Configuration that determines the serialization format
/// * `value` - The byte slice to serialize
///
/// # Returns
///
/// Returns `std::io::Result<()>` indicating success or failure
pub fn serde_bytes<W, F>(
    writer: &mut W,
    formatter: &mut F,
    config: &Config,
    value: &[u8],
) -> std::io::Result<()>
where
    W: ?Sized + Write,
    F: serde_json::ser::Formatter,
{
    match config.bytes_format {
        BytesFormat::Default => serde_bytes_array(writer, formatter, value),
        BytesFormat::Hex => serde_bytes_hex(writer, formatter, config, value),
        BytesFormat::Base64 => serde_bytes_base64(writer, formatter, false, value),
        BytesFormat::Base64UrlSafe => serde_bytes_base64(writer, formatter, true, value),
    }
}

/// Serializes bytes as an array of numbers [1, 2, 3]
pub fn serde_bytes_array<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &[u8],
) -> std::io::Result<()>
where
    W: ?Sized + Write,
    F: serde_json::ser::Formatter,
{
    formatter.write_byte_array(writer, value)
}

/// Serializes bytes as a hexadecimal string "0x1234..." or "1234..."
pub fn serde_bytes_hex<W, F>(
    writer: &mut W,
    formatter: &mut F,
    config: &Config,
    value: &[u8],
) -> std::io::Result<()>
where
    W: ?Sized + Write,
    F: serde_json::ser::Formatter,
{
    let hex_str = hex::encode(value);
    
    formatter.begin_string(writer)?;
    if config.hex_prefix {
        formatter.write_string_fragment(writer, "0x")?;
    }
    formatter.write_string_fragment(writer, &hex_str)?;
    formatter.end_string(writer)?;
    Ok(())
}

/// Serializes bytes as a Base64 string
///
/// # Arguments
///
/// * `url_safe` - If true, uses URL-safe Base64 encoding, otherwise uses standard Base64
pub fn serde_bytes_base64<W, F>(
    writer: &mut W,
    formatter: &mut F,
    url_safe: bool,
    value: &[u8],
) -> std::io::Result<()>
where
    W: ?Sized + Write,
    F: serde_json::ser::Formatter,
{
    use base64::{Engine as _, engine::general_purpose};
    let encoded = if url_safe {
        general_purpose::URL_SAFE.encode(value)
    } else {
        general_purpose::STANDARD.encode(value)
    };
    formatter.begin_string(writer)?;
    formatter.write_string_fragment(writer, &encoded)?;
    formatter.end_string(writer)?;
    Ok(())
}
