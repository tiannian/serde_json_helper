// Serializer wrapper for serde_json::value::Serializer

use crate::{BytesFormat, Config};

/// A wrapper around `serde_json::value::Serializer` that implements `Serializer`
pub struct Serializer<'a> {
    /// The internal `serde_json::value::Serializer`
    pub inner: serde_json::value::Serializer,
    /// Configuration for serialization
    pub config: &'a Config,
}

impl<'a> Serializer<'a> {
    /// Creates a new `Serializer` with custom config
    pub fn with_config(config: &'a Config) -> Self {
        Serializer {
            inner: serde_json::value::Serializer,
            config,
        }
    }
}

impl<'a> serde::Serializer for Serializer<'a> {
    type Ok = serde_json::Value;
    type Error = serde_json::Error;
    type SerializeSeq = <serde_json::value::Serializer as serde::Serializer>::SerializeSeq;
    type SerializeTuple = <serde_json::value::Serializer as serde::Serializer>::SerializeTuple;
    type SerializeTupleStruct = <serde_json::value::Serializer as serde::Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant = <serde_json::value::Serializer as serde::Serializer>::SerializeTupleVariant;
    type SerializeMap = <serde_json::value::Serializer as serde::Serializer>::SerializeMap;
    type SerializeStruct = <serde_json::value::Serializer as serde::Serializer>::SerializeStruct;
    type SerializeStructVariant = <serde_json::value::Serializer as serde::Serializer>::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.config.bytes_format {
            BytesFormat::Default => {
                // Serialize as array of numbers [1, 2, 3]
                let array: Vec<serde_json::Value> = v.iter()
                    .map(|&b| serde_json::Value::Number(serde_json::Number::from(b as u64)))
                    .collect();
                Ok(serde_json::Value::Array(array))
            }
            BytesFormat::Hex => {
                // Serialize as hexadecimal string "0x1234..." or "1234..."
                let hex_str = hex::encode(v);
                let mut result = String::new();
                if self.config.hex_prefix {
                    result.push_str("0x");
                }
                result.push_str(&hex_str);
                Ok(serde_json::Value::String(result))
            }
            BytesFormat::Base64 => {
                // Serialize as Base64 string
                use base64::{Engine as _, engine::general_purpose};
                let encoded = general_purpose::STANDARD.encode(v);
                Ok(serde_json::Value::String(encoded))
            }
            BytesFormat::Base64UrlSafe => {
                // Serialize as URL-safe Base64 string
                use base64::{Engine as _, engine::general_purpose};
                let encoded = general_purpose::URL_SAFE.encode(v);
                Ok(serde_json::Value::String(encoded))
            }
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.inner.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner.serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.inner.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.inner.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner.serialize_struct_variant(name, variant_index, variant, len)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + std::fmt::Display,
    {
        self.inner.collect_str(value)
    }
}
