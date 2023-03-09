use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Url(String);

impl Url {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Url {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for Url {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Url {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if validator::validate_url(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

impl From<String> for Url {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Serialize for Url {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
