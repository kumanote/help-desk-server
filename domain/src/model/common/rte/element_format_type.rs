use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const LEFT: &'static str = "left";
const START: &'static str = "start";
const CENTER: &'static str = "center";
const RIGHT: &'static str = "right";
const END: &'static str = "end";
const JUSTIFY: &'static str = "justify";
const DEFAULT: &'static str = "";

#[derive(Debug, Clone, PartialEq)]
pub enum RteElementFormatType {
    Left,
    Start,
    Center,
    Right,
    End,
    Justify,
    Default,
}

impl RteElementFormatType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Left => LEFT,
            Self::Start => START,
            Self::Center => CENTER,
            Self::Right => RIGHT,
            Self::End => END,
            Self::Justify => JUSTIFY,
            Self::Default => DEFAULT,
        }
    }
}

impl AsRef<str> for RteElementFormatType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteElementFormatType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RteElementFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteElementFormatType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteElementFormatType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            LEFT => Ok(Self::Left),
            START => Ok(Self::Start),
            CENTER => Ok(Self::Center),
            RIGHT => Ok(Self::Right),
            END => Ok(Self::End),
            JUSTIFY => Ok(Self::Justify),
            DEFAULT => Ok(Self::Default),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteElementFormatType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteElementFormatType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
