use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const ENGLISH: &'static str = "en";
const JAPANESE: &'static str = "ja";
const VIETNAMESE: &'static str = "vi";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    English,
    Japanese,
    Vietnamese,
}

impl Default for Locale {
    fn default() -> Self {
        Self::English
    }
}

impl Locale {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn all() -> Vec<Self> {
        vec![Self::English, Self::Japanese, Self::Vietnamese]
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        match self {
            Self::English => ENGLISH,
            Self::Japanese => JAPANESE,
            Self::Vietnamese => VIETNAMESE,
        }
    }
}

impl Deref for Locale {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for Locale {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for Locale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            ENGLISH => Ok(Self::English),
            JAPANESE => Ok(Self::Japanese),
            VIETNAMESE => Ok(Self::Vietnamese),
            _ => Err(Error::UnsupportedLocale {
                value: s.to_owned(),
            }),
        }
    }
}

impl From<String> for Locale {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap_or_default()
    }
}

impl Serialize for Locale {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
