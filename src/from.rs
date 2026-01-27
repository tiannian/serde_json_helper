use serde::Deserialize;
use serde_json::{Result, de::Read};

use crate::{Config, de::Deserializer};

fn from_trait<'de, R, T>(read: R, config: &'de Config) -> Result<T>
where
    R: Read<'de>,
    T: Deserialize<'de>,
{
    let mut de = Deserializer::with_config(serde_json::Deserializer::new(read), config);

    let value = serde::de::Deserialize::deserialize(&mut de)?;

    let mut inner = de.inner;

    inner.end()?;

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
