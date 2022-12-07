use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const LINK: &'static str = "LINK";
const IMAGE: &'static str = "IMAGE";

#[derive(Debug, Clone, PartialEq)]
pub enum RteEntityType {
    Link,
    Image,
    Custom(String),
}

impl RteEntityType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for RteEntityType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Link => LINK,
            Self::Image => IMAGE,
            Self::Custom(s) => &s,
        }
    }
}

impl Deref for RteEntityType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for RteEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteEntityType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteEntityType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            LINK => Ok(Self::Link),
            IMAGE => Ok(Self::Image),
            _ => Ok(Self::Custom(s.to_owned())),
        }
    }
}

impl Serialize for RteEntityType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteEntityType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
