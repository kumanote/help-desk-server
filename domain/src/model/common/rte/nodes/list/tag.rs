use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const UL: &'static str = "ul";
const OL: &'static str = "ol";

#[derive(Debug, Clone, PartialEq)]
pub enum RteListNodeTag {
    UnorderedList,
    OrderedList,
}

impl RteListNodeTag {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UnorderedList => UL,
            Self::OrderedList => OL,
        }
    }
}

impl AsRef<str> for RteListNodeTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteListNodeTag {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RteListNodeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteListNodeTag {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteListNodeTag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            UL => Ok(Self::UnorderedList),
            OL => Ok(Self::OrderedList),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteListNodeTag {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteListNodeTag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
