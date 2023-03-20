use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const NUMBER: &'static str = "number";
const BULLET: &'static str = "bullet";
const CHECK: &'static str = "check";

#[derive(Debug, Clone, PartialEq)]
pub enum RteListType {
    Number,
    Bullet,
    Check,
}

impl RteListType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Number => NUMBER,
            Self::Bullet => BULLET,
            Self::Check => CHECK,
        }
    }
}

impl AsRef<str> for RteListType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteListType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RteListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteListType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteListType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            NUMBER => Ok(Self::Number),
            BULLET => Ok(Self::Bullet),
            CHECK => Ok(Self::Check),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteListType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteListType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
