use crate::{Error, Result};
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const PATTERN: &'static str = "^[a-zA-Z0-9_-]{3,55}$";
static REGEX: OnceCell<Regex> = OnceCell::new();

/// slug is used as part of URLs.
#[derive(Clone, Debug, PartialEq)]
pub struct Slug(String);

impl Slug {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Slug {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for Slug {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Slug {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let re = REGEX.get_or_init(|| Regex::new(PATTERN).unwrap());
        if re.is_match(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

impl From<String> for Slug {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Serialize for Slug {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Slug {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_slugs() {
        let samples = vec![
            "123",
            "abc-DEF_123",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123",
        ];
        for s in samples {
            let slug = Slug::from_str(s).unwrap();
            assert_eq!(slug.as_str(), s);
        }
    }

    #[test]
    fn test_invalid_slugs() {
        let samples = vec![
            "12",
            "abc-DEF_123%",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234",
        ];
        for s in samples {
            assert!(Slug::from_str(s).is_err());
        }
    }
}
