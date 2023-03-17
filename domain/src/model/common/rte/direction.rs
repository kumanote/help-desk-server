use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const LTR: &'static str = "ltr";
const RTL: &'static str = "rtl";

#[derive(Debug, Clone, PartialEq)]
pub enum RteDirection {
    LTR,
    RTL,
}

impl RteDirection {
    pub fn as_str(&self) -> &str {
        match self {
            Self::LTR => LTR,
            Self::RTL => RTL,
        }
    }
}

impl AsRef<str> for RteDirection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for RteDirection {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for RteDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteDirection {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteDirection {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            LTR => Ok(Self::LTR),
            RTL => Ok(Self::RTL),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteDirection {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteDirection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
