use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const EN_US: &'static str = "en_US";
const JA_JP: &'static str = "ja_JP";
const VI_VN: &'static str = "vi_VN";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentLocale {
    EnUs,
    JaJp,
    ViVN,
}

impl Default for ContentLocale {
    fn default() -> Self {
        Self::EnUs
    }
}

impl ContentLocale {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn all() -> Vec<Self> {
        vec![Self::EnUs, Self::JaJp, Self::ViVN]
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::EnUs => "English (US)",
            Self::JaJp => "日本語",
            Self::ViVN => "Tiếng Việt",
        }
    }
}

impl AsRef<str> for ContentLocale {
    fn as_ref(&self) -> &str {
        match self {
            Self::EnUs => EN_US,
            Self::JaJp => JA_JP,
            Self::ViVN => VI_VN,
        }
    }
}

impl Deref for ContentLocale {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ContentLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for ContentLocale {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for ContentLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            EN_US => Ok(Self::EnUs),
            JA_JP => Ok(Self::JaJp),
            VI_VN => Ok(Self::ViVN),
            _ => Err(Error::UnsupportedLocale {
                value: s.to_owned(),
            }),
        }
    }
}

impl From<String> for ContentLocale {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap_or_default()
    }
}

impl Serialize for ContentLocale {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ContentLocale {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
