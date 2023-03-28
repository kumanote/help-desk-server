use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const OPEN: &'static str = "open";
const PENDING: &'static str = "pending";
const CLOSED: &'static str = "closed";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InquiryThreadStatus {
    Open,
    Pending,
    Closed,
}

impl Default for InquiryThreadStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl InquiryThreadStatus {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Open, Self::Pending, Self::Closed]
    }
}

impl AsRef<str> for InquiryThreadStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Open => OPEN,
            Self::Pending => PENDING,
            Self::Closed => CLOSED,
        }
    }
}

impl Deref for InquiryThreadStatus {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for InquiryThreadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for InquiryThreadStatus {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for InquiryThreadStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            OPEN => Ok(Self::Open),
            PENDING => Ok(Self::Pending),
            CLOSED => Ok(Self::Closed),
            _ => Err(Error::UnsupportedLocale {
                value: s.to_owned(),
            }),
        }
    }
}

impl From<String> for InquiryThreadStatus {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap_or_default()
    }
}

impl Serialize for InquiryThreadStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InquiryThreadStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
