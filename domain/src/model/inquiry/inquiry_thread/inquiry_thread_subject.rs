use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

const MAX_LENGTH: usize = 200;

#[derive(Clone, Debug, PartialEq)]
pub struct InquiryThreadSubject(String);

impl InquiryThreadSubject {
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for InquiryThreadSubject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for InquiryThreadSubject {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Into<String> for InquiryThreadSubject {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for InquiryThreadSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InquiryThreadSubject {
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

fn substring(s: &str, max_length: usize) -> String {
    let chars: Vec<&str> = s.graphemes(true).collect();
    if chars.len() <= max_length {
        chars.join("")
    } else {
        chars[..max_length].join("")
    }
}

impl From<&line::events::messages::MessageType> for InquiryThreadSubject {
    fn from(value: &line::events::messages::MessageType) -> Self {
        match value {
            line::events::messages::MessageType::TextMessage(inner) => {
                Self(substring(&inner.text, MAX_LENGTH))
            },
            line::events::messages::MessageType::ImageMessage(_) => {
                Self("Image message via line".to_owned())
            },
            line::events::messages::MessageType::VideoMessage(_) => {
                Self("Video message via line".to_owned())
            },
            line::events::messages::MessageType::AudioMessage(_) => {
                Self("Audio message via line".to_owned())
            },
            line::events::messages::MessageType::FileMessage(inner) => {
                let subject = format!("File: {}", inner.file_name);
                Self(substring(&subject, MAX_LENGTH))
            },
            line::events::messages::MessageType::LocationMessage(inner) => {
                let subject = format!("Location: {} {}", inner.title, inner.address);
                Self(substring(&subject, MAX_LENGTH))
            },
            line::events::messages::MessageType::StickerMessage(_) => {
                Self("Sticker message via line".to_owned())
            },
        }
    }
}

impl From<String> for InquiryThreadSubject {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Serialize for InquiryThreadSubject {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InquiryThreadSubject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_substring() {
        let samples = vec![("123", "123"), ("abc-DEF_123", "abc"), ("a̐éö̲\r\n", "a̐éö̲")];
        for (s, e) in samples {
            let r = substring(s, 3);
            assert_eq!(r, e);
        }
    }
}
