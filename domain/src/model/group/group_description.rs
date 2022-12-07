use crate::{Error, Result};
use std::fmt;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

const MAX_LENGTH: usize = 1000;

#[derive(Clone, Debug, PartialEq)]
pub struct GroupDescription(Option<String>);

impl GroupDescription {
    #[inline]
    pub fn has_content(&self) -> bool {
        self.0.is_some()
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub const fn as_ref(&self) -> Option<&String> {
        self.0.as_ref()
    }

    #[inline]
    pub fn as_deref(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

impl Into<Option<String>> for GroupDescription {
    fn into(self) -> Option<String> {
        self.0
    }
}

impl fmt::Display for GroupDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_deref().unwrap_or_default())
    }
}

impl FromStr for GroupDescription {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            Ok(Self(None))
        } else {
            let length = s.graphemes(true).count();
            if length <= MAX_LENGTH {
                Ok(Self(Some(s.to_owned())))
            } else {
                Err(Error::InvalidFormat)
            }
        }
    }
}

impl From<Option<String>> for GroupDescription {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
