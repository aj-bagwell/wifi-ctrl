use std::collections::HashMap;
use std::fmt::Display;

use serde::de::value::MapDeserializer;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ConfigError {
    #[error("Missing '=' delimiter in config line")]
    MissingDelimterEqual,
    #[error("escape code is not made up of valid hex code")]
    InvalidEscape,
    #[error("escape code is incomplete")]
    IncompleteEscape,
    #[error("escaped value is not valid uft8 after unescaping")]
    NonUtf8Escape,
    #[error("Value could not be decoded")]
    SerdeError(String),
}

impl serde::de::Error for ConfigError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}

pub(crate) fn to_map(response: &str) -> Result<HashMap<&str, String>, ConfigError> {
    let mut map = HashMap::new();
    for line in response.trim().lines() {
        let (k, v) = line
            .split_once('=')
            .ok_or(ConfigError::MissingDelimterEqual)?;
        map.insert(k, unprintf(v)?);
    }
    Ok(map)
}

pub(crate) fn deserialize_str<'de, T: Deserialize<'de>>(response: &str) -> Result<T, ConfigError> {
    let map = to_map(response)?;
    T::deserialize(MapDeserializer::new(map.into_iter()))
}

pub(crate) fn unprintf(escaped: &str) -> std::result::Result<String, ConfigError> {
    let mut bytes = escaped.as_bytes().iter().copied();
    let mut unescaped = vec![];
    // undo "printf_encode"
    loop {
        unescaped.push(match bytes.next() {
            Some(b'\\') => match bytes.next().ok_or(ConfigError::IncompleteEscape)? {
                b'n' => b'\n',
                b'r' => b'\r',
                b't' => b'\t',
                b'e' => b'\x1b',
                b'x' => {
                    let hex = [
                        bytes.next().ok_or(ConfigError::IncompleteEscape)?,
                        bytes.next().ok_or(ConfigError::IncompleteEscape)?,
                    ];
                    u8::from_str_radix(
                        std::str::from_utf8(&hex).or(Err(ConfigError::InvalidEscape))?,
                        16,
                    )
                    .or(Err(ConfigError::InvalidEscape))?
                }
                c => c,
            },
            Some(c) => c,
            None => break,
        })
    }
    String::from_utf8(unescaped).or(Err(ConfigError::NonUtf8Escape))
}
