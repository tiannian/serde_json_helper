use serde::Deserialize;
use serde_json::{Result, de::Read};

use crate::{Config, de::Deserializer};

fn from_trait<'de, R, T>(read: R, config: Config) -> Result<T>
where
    R: Read<'de>,
    T: Deserialize<'de>,
{
    let mut de = Deserializer::with_config(serde_json::Deserializer::new(read), config.clone());

    let value = serde::de::Deserialize::deserialize(&mut de)?;

    de.inner.end()?;
    Ok(value)
}

pub fn from_reader<R, T>(rdr: R, config: Config) -> Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    from_trait(serde_json::de::IoRead::new(rdr), config)
}