// Serialization functions with configuration

use crate::formatter::{CompactFormatter, PrettyFormatter};
use crate::Config;
use std::io::Write;

/// Serializes a value to a JSON string with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_string, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_string(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_string<T>(value: &T, config: &Config) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let formatter = CompactFormatter::with_config(config.clone());
    let mut writer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(writer).unwrap())
}

/// Serializes a value to a pretty-printed JSON string with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_string_pretty, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_string_pretty(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_string_pretty<T>(value: &T, config: &Config) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let formatter = PrettyFormatter::with_config(config.clone());
    let mut writer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(writer).unwrap())
}

/// Serializes a value to a JSON byte vector with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_vec, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_vec(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_vec<T>(value: &T, config: &Config) -> serde_json::Result<Vec<u8>>
where
    T: ?Sized + serde::Serialize,
{
    let formatter = CompactFormatter::with_config(config.clone());
    let mut writer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer)?;
    Ok(writer)
}

/// Serializes a value to a pretty-printed JSON byte vector with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_vec_pretty, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_vec_pretty(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_vec_pretty<T>(value: &T, config: &Config) -> serde_json::Result<Vec<u8>>
where
    T: ?Sized + serde::Serialize,
{
    let formatter = PrettyFormatter::with_config(config.clone());
    let mut writer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut serializer)?;
    Ok(writer)
}

/// Serializes a value to a JSON writer with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_writer, Config};
/// use std::io::stdout;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// to_writer(&mut stdout(), &vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_writer<W, T>(
    writer: &mut W,
    value: &T,
    config: &Config,
) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = CompactFormatter::with_config(config.clone());
    let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
    value.serialize(&mut serializer)
}

/// Serializes a value to a pretty-printed JSON writer with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_writer_pretty, Config};
/// use std::io::stdout;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// to_writer_pretty(&mut stdout(), &vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_writer_pretty<W, T>(
    writer: &mut W,
    value: &T,
    config: &Config,
) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = PrettyFormatter::with_config(config.clone());
    let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
    value.serialize(&mut serializer)
}

/// Converts a `serde_json::Value` to a JSON string with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_value, Config};
/// use serde_json::json;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let value = json!({"data": [1, 2, 3]});
/// let json = to_value(&value, &config).unwrap();
/// ```
pub fn to_value(
    value: &serde_json::Value,
    config: &Config,
) -> serde_json::Result<String> {
    to_string(value, config)
}

/// Converts a `serde_json::Value` to a pretty-printed JSON string with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_value_pretty, Config};
/// use serde_json::json;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let value = json!({"data": [1, 2, 3]});
/// let json = to_value_pretty(&value, &config).unwrap();
/// ```
pub fn to_value_pretty(
    value: &serde_json::Value,
    config: &Config,
) -> serde_json::Result<String> {
    to_string_pretty(value, config)
}
