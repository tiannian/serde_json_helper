use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Result, de::Read};

use crate::{Config, de::Deserializer};

fn from_trait<'de, R, T>(read: R, config: &'de Config) -> Result<T>
where
    R: Read<'de>,
    T: Deserialize<'de>,
{
    let mut serde_json_de = serde_json::Deserializer::new(read);
    let de = Deserializer::with_config(&mut serde_json_de, config);

    let value = serde::de::Deserialize::deserialize(de)?;

    serde_json_de.end()?;

    Ok(value)
}

pub fn from_reader<R, T>(rdr: R, config: &Config) -> Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    from_trait(serde_json::de::IoRead::new(rdr), config)
}

pub fn from_slice<'a, T>(v: &'a [u8], config: &'a Config) -> Result<T>
where
    T: Deserialize<'a>,
{
    from_trait(serde_json::de::SliceRead::new(v), config)
}

pub fn from_str<'a, T>(s: &'a str, config: &'a Config) -> Result<T>
where
    T: Deserialize<'a>,
{
    from_trait(serde_json::de::StrRead::new(s), config)
}

pub fn from_value<T>(value: serde_json::Value, config: &Config) -> Result<T>
where
    T: DeserializeOwned,
{
    let de = Deserializer::with_config(value, config);

    let value = serde::de::Deserialize::deserialize(de)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_from_str_hex_without_prefix_to_vec_u8() {
        let config = Config::default().set_bytes_hex().disable_hex_prefix();

        #[derive(Deserialize, Debug)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let json = r#"{"data":"0000ff"}"#;
        let result: Result<TestStruct> = from_str(json, &config);
        assert_eq!(result.unwrap().data, vec![0, 0, 255]);
    }

    #[test]
    fn test_from_str_hex_with_prefix_to_vec_u8() {
        let config = Config::default().set_bytes_hex().enable_hex_prefix();

        #[derive(Deserialize, Debug)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let json = json!({
            "data": "0x0000ff"
        });

        let result: Result<TestStruct> = from_value(json, &config);
        assert_eq!(result.unwrap().data, vec![0, 0, 255]);
    }
}
