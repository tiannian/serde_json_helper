// Serialization functions with configuration

use crate::Config;
use crate::formatter::{CompactFormatter, PrettyFormatter};
use crate::ser::value::Serializer;
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
    let bytes = to_vec(value, config)?;
    Ok(String::from_utf8(bytes).unwrap())
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
    let bytes = to_vec_pretty(value, config)?;
    Ok(String::from_utf8(bytes).unwrap())
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
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value, config)?;
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
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value, config)?;
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
pub fn to_writer<W, T>(writer: &mut W, value: &T, config: &Config) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = CompactFormatter::with_config(config);
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
pub fn to_writer_pretty<W, T>(writer: &mut W, value: &T, config: &Config) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = PrettyFormatter::with_config(config);
    let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
    value.serialize(&mut serializer)
}

/// Serializes a value to a `serde_json::Value` with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{to_value, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let value = to_value(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_value<T>(value: &T, config: &Config) -> serde_json::Result<serde_json::Value>
where
    T: ?Sized + serde::Serialize,
{
    let serializer = Serializer::with_config(config);
    value.serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_bytes_default() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_default();
        let result = to_string(&test_data, &config).unwrap();
        println!("Bytes Default format: {}", result);
    }

    #[test]
    fn test_to_string_bytes_hex_without_prefix() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_hex().disable_hex_prefix();
        let result = to_string(&test_data, &config).unwrap();
        println!("Bytes Hex format (no prefix): {}", result);
    }

    #[test]
    fn test_to_string_bytes_hex_with_prefix() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_hex().enable_hex_prefix();
        let result = to_string(&test_data, &config).unwrap();
        println!("Bytes Hex format (with 0x prefix): {}", result);
    }

    #[test]
    fn test_to_string_bytes_base64() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_base64();
        let result = to_string(&test_data, &config).unwrap();
        println!("Bytes Base64 format: {}", result);
    }

    #[test]
    fn test_to_string_bytes_base64_url_safe() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_base64_url_safe();
        let result = to_string(&test_data, &config).unwrap();
        println!("Bytes Base64 URL-safe format: {}", result);
    }

    #[test]
    fn test_to_string_bytes_empty() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct { data: vec![] };

        println!("\n=== Testing empty bytes ===");

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        println!("Empty bytes Default format: {}", result_default);

        let config_hex = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        println!("Empty bytes Hex format (no prefix): {}", result_hex);

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        println!(
            "Empty bytes Hex format (with prefix): {}",
            result_hex_prefix
        );

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Empty bytes Base64 format: {}", result_base64);

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        println!("Empty bytes Base64 URL-safe format: {}", result_base64_url);
    }

    #[test]
    fn test_to_string_bytes_single_byte() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct { data: vec![0u8] };

        println!("\n=== Testing single byte (0) ===");

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        println!("Single byte (0) Default format: {}", result_default);

        let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
        println!(
            "Single byte (0) Hex format (no prefix): {}",
            result_hex_no_prefix
        );

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        println!(
            "Single byte (0) Hex format (with prefix): {}",
            result_hex_prefix
        );

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Single byte (0) Base64 format: {}", result_base64);
    }

    #[test]
    fn test_to_string_bytes_large() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: (0u8..=255u8).collect(),
        };

        println!("\n=== Testing large bytes (0-255) ===");

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        println!(
            "Large bytes Default format length: {}",
            result_default.len()
        );
        println!(
            "Large bytes Default format (first 100 chars): {}",
            &result_default[..result_default.len().min(100)]
        );

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        println!("Large bytes Hex format (with prefix): {}", result_hex);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Large bytes Base64 format: {}", result_base64);

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        println!("Large bytes Base64 URL-safe format: {}", result_base64_url);
    }

    #[test]
    fn test_to_string_bytes_in_struct() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
            name: String,
        }

        let test_data = TestStruct {
            data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello" in ASCII
            name: "test".to_string(),
        };

        println!("\n=== Testing bytes in struct ===");

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        println!("Struct with bytes Default format: {}", result_default);

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        println!("Struct with bytes Hex format (with prefix): {}", result_hex);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Struct with bytes Base64 format: {}", result_base64);
    }

    #[test]
    fn test_to_string_bytes_in_nested_structure() {
        #[derive(serde::Serialize)]
        struct Nested {
            #[serde(with = "serde_bytes")]
            inner: Vec<u8>,
        }

        #[derive(serde::Serialize)]
        struct Outer {
            nested: Nested,
            #[serde(with = "serde_bytes")]
            bytes: Vec<u8>,
        }

        let data = Outer {
            nested: Nested {
                inner: vec![1, 2, 3],
            },
            bytes: vec![4, 5, 6],
        };

        println!("\n=== Testing bytes in nested structure ===");

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&data, &config_default).unwrap();
        println!("Nested structure Default format: {}", result_default);

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&data, &config_hex).unwrap();
        println!("Nested structure Hex format (with prefix): {}", result_hex);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&data, &config_base64).unwrap();
        println!("Nested structure Base64 format: {}", result_base64);
    }

    #[test]
    fn test_to_string_bytes_special_values() {
        let test_cases = vec![
            (vec![0u8], "zero byte"),
            (vec![255u8], "max byte"),
            (vec![0u8, 255u8], "zero and max"),
            (vec![0x00, 0xFF, 0x7F, 0x80], "boundary values"),
            (vec![0x01, 0x02, 0x03, 0x04, 0x05], "sequential bytes"),
        ];

        println!("\n=== Testing special byte values ===");

        for (bytes, description) in test_cases {
            #[derive(serde::Serialize)]
            struct TestStruct {
                #[serde(with = "serde_bytes")]
                data: Vec<u8>,
            }

            let test_data = TestStruct { data: bytes };

            println!("\n--- Testing: {} ---", description);

            let config_default = Config::default().set_bytes_default();
            let result_default = to_string(&test_data, &config_default).unwrap();
            println!("Default: {}", result_default);

            let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
            let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
            println!("Hex (no prefix): {}", result_hex_no_prefix);

            let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
            let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
            println!("Hex (with prefix): {}", result_hex_prefix);

            let config_base64 = Config::default().set_bytes_base64();
            let result_base64 = to_string(&test_data, &config_base64).unwrap();
            println!("Base64: {}", result_base64);

            let config_base64_url = Config::default().set_bytes_base64_url_safe();
            let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
            println!("Base64 URL-safe: {}", result_base64_url);
        }
    }

    #[test]
    fn test_to_string_bytes_all_formats() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        };

        println!(
            "\n=== Testing all formats with same bytes: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0] ==="
        );

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        println!("Default: {}", result_default);

        let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
        println!("Hex (no prefix): {}", result_hex_no_prefix);

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        println!("Hex (with prefix): {}", result_hex_prefix);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Base64: {}", result_base64);

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        println!("Base64 URL-safe: {}", result_base64_url);
    }

    #[test]
    fn test_to_string_bytes_multiple_fields() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            field1: Vec<u8>,
            #[serde(with = "serde_bytes")]
            field2: Vec<u8>,
            name: String,
        }

        let test_data = TestStruct {
            field1: vec![1, 2, 3],
            field2: vec![4, 5, 6],
            name: "test".to_string(),
        };

        println!("\n=== Testing multiple bytes fields ===");

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        println!("Multiple bytes fields Hex format: {}", result_hex);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        println!("Multiple bytes fields Base64 format: {}", result_base64);
    }
}
