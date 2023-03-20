use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const H1: &'static str = "h1";
const H2: &'static str = "h2";
const H3: &'static str = "h3";
const H4: &'static str = "h4";
const H5: &'static str = "h5";
const H6: &'static str = "h6";

#[derive(Debug, Clone, PartialEq)]
pub enum RteHeadingNodeTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl RteHeadingNodeTag {
    pub fn as_str(&self) -> &str {
        match self {
            Self::H1 => H1,
            Self::H2 => H2,
            Self::H3 => H3,
            Self::H4 => H4,
            Self::H5 => H5,
            Self::H6 => H6,
        }
    }
}

impl AsRef<str> for RteHeadingNodeTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteHeadingNodeTag {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RteHeadingNodeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteHeadingNodeTag {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteHeadingNodeTag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            H1 => Ok(Self::H1),
            H2 => Ok(Self::H2),
            H3 => Ok(Self::H3),
            H4 => Ok(Self::H4),
            H5 => Ok(Self::H5),
            H6 => Ok(Self::H6),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteHeadingNodeTag {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteHeadingNodeTag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
