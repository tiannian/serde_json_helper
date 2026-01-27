// Compact formatter for JSON

use crate::{Config, ser_bytes::ser_bytes};
use std::io::Write;

/// Compact formatter for JSON serialization
pub struct CompactFormatter<'a> {
    /// The underlying serde_json compact formatter
    formatter: serde_json::ser::CompactFormatter,
    /// Configuration for the formatter
    config: &'a Config,
}

impl<'a> CompactFormatter<'a> {
    /// Creates a new CompactFormatter with the specified configuration
    pub fn with_config(config: &'a Config) -> Self {
        CompactFormatter {
            formatter: serde_json::ser::CompactFormatter,
            config,
        }
    }
}

impl<'a> serde_json::ser::Formatter for CompactFormatter<'a> {
    fn write_null<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_null(writer)
    }

    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_bool(writer, value)
    }

    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i8(writer, value)
    }

    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i16(writer, value)
    }

    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i32(writer, value)
    }

    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i64(writer, value)
    }

    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i128(writer, value)
    }

    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_u8(writer, value)
    }

    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_u16(writer, value)
    }

    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_u32(writer, value)
    }

    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_u64(writer, value)
    }

    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_u128(writer, value)
    }

    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_f32(writer, value)
    }

    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_f64(writer, value)
    }

    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_number_str(writer, value)
    }

    fn begin_string<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_string(writer)
    }

    fn end_string<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_string(writer)
    }

    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_string_fragment(writer, fragment)
    }

    fn write_char_escape<W>(
        &mut self,
        writer: &mut W,
        char_escape: serde_json::ser::CharEscape,
    ) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_char_escape(writer, char_escape)
    }

    fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        ser_bytes(writer, &mut self.formatter, &self.config, value)
    }

    fn begin_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_array(writer)
    }

    fn end_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_array(writer)
    }

    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_array_value(writer, first)
    }

    fn end_array_value<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_array_value(writer)
    }

    fn begin_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_object(writer)
    }

    fn end_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_object(writer)
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_object_key(writer, first)
    }

    fn end_object_key<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_object_key(writer)
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_object_value(writer)
    }

    fn end_object_value<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_object_value(writer)
    }

    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> std::io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_raw_fragment(writer, fragment)
    }
}
