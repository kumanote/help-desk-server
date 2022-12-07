use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

const MIN_LENGTH: usize = 1;
const MAX_LENGTH: usize = 50;

#[derive(Clone, Debug, PartialEq)]
pub struct FaqItemTitle(String);

impl FaqItemTitle {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for FaqItemTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for FaqItemTitle {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for FaqItemTitle {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for FaqItemTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FaqItemTitle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let length = s.graphemes(true).count();
        if MIN_LENGTH <= length && length <= MAX_LENGTH {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

impl From<String> for FaqItemTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Serialize for FaqItemTitle {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FaqItemTitle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
