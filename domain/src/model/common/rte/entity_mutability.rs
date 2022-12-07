use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const MUTABLE: &'static str = "MUTABLE";
const IMMUTABLE: &'static str = "IMMUTABLE";
const SEGMENTED: &'static str = "SEGMENTED";

/// Possible "mutability" options for an entity. This refers to the behavior
/// that should occur when inserting or removing characters in a text range
/// with an entity applied to it.
///
/// `MUTABLE`:
///   The text range can be modified freely. Generally used in cases where
///   the text content and the entity do not necessarily have a direct
///   relationship. For instance, the text and URI for a link may be completely
///   different. The user is allowed to edit the text as needed, and the entity
///   is preserved and applied to any characters added within the range.
///
/// `IMMUTABLE`:
///   Not to be confused with immutable data structures used to represent the
///   state of the editor. Immutable entity ranges cannot be modified in any
///   way. Adding characters within the range will remove the entity from the
///   entire range. Deleting characters will delete the entire range. Example:
///   Facebook Page mentions.
///
/// `SEGMENTED`:
///   Segmented entities allow the removal of partial ranges of text, as
///   separated by a delimiter. Adding characters within the range will remove
///   the entity from the entire range. Deleting characters within a segmented
///   entity will delete only the segments affected by the deletion. Example:
///   Facebook User mentions.
#[derive(Debug, Clone, PartialEq)]
pub enum RteEntityMutability {
    Mutable,
    Immutable,
    Segmented,
}

impl RteEntityMutability {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for RteEntityMutability {
    fn as_ref(&self) -> &str {
        match self {
            Self::Mutable => MUTABLE,
            Self::Immutable => IMMUTABLE,
            Self::Segmented => SEGMENTED,
        }
    }
}

impl Deref for RteEntityMutability {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for RteEntityMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteEntityMutability {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteEntityMutability {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            MUTABLE => Ok(Self::Mutable),
            IMMUTABLE => Ok(Self::Immutable),
            SEGMENTED => Ok(Self::Segmented),
            _ => Err(Error::UnsupportedRteValue {
                value: s.to_owned(),
            }),
        }
    }
}

impl Serialize for RteEntityMutability {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteEntityMutability {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
