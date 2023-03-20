use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const NORMAL: &'static str = "normal";
const TOKEN: &'static str = "token";
const SEGMENTED: &'static str = "segmented";

#[derive(Debug, Clone, PartialEq)]
pub enum RteTextModeType {
    Normal,
    Token,
    Segmented,
}

impl RteTextModeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => NORMAL,
            Self::Token => TOKEN,
            Self::Segmented => SEGMENTED,
        }
    }
}

impl AsRef<str> for RteTextModeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteTextModeType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RteTextModeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteTextModeType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteTextModeType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            NORMAL => Ok(Self::Normal),
            TOKEN => Ok(Self::Token),
            SEGMENTED => Ok(Self::Segmented),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteTextModeType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteTextModeType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
