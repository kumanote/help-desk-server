use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const BOLD: &'static str = "BOLD";
const CODE: &'static str = "CODE";
const ITALIC: &'static str = "ITALIC";
const STRIKETHROUGH: &'static str = "STRIKETHROUGH";
const UNDERLINE: &'static str = "UNDERLINE";

#[derive(Debug, Clone, PartialEq)]
pub enum RteInlineStyleType {
    Bold,
    Code,
    Italic,
    Strikethrough,
    Underline,
}

impl RteInlineStyleType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for RteInlineStyleType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Bold => BOLD,
            Self::Code => CODE,
            Self::Italic => ITALIC,
            Self::Strikethrough => STRIKETHROUGH,
            Self::Underline => UNDERLINE,
        }
    }
}

impl Deref for RteInlineStyleType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for RteInlineStyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteInlineStyleType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteInlineStyleType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            BOLD => Ok(Self::Bold),
            CODE => Ok(Self::Code),
            ITALIC => Ok(Self::Italic),
            STRIKETHROUGH => Ok(Self::Strikethrough),
            UNDERLINE => Ok(Self::Underline),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteInlineStyleType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteInlineStyleType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
