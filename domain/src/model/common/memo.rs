use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

const MAX_LENGTH: usize = 4000;

#[derive(Clone, Debug, PartialEq)]
pub struct Memo(String);

impl Memo {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Memo {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Memo {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Into<String> for Memo {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Memo {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let length = s.graphemes(true).count();
        if length <= MAX_LENGTH {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

impl From<String> for Memo {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Serialize for Memo {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Memo {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
