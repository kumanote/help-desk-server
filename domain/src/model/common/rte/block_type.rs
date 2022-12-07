use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const HEADER_ONE: &'static str = "header-one";
const HEADER_TWO: &'static str = "header-two";
const HEADER_THREE: &'static str = "header-three";
const HEADER_FOUR: &'static str = "header-four";
const HEADER_FIVE: &'static str = "header-five";
const HEADER_SIX: &'static str = "header-six";
const SECTION: &'static str = "section";
const ARTICLE: &'static str = "article";
const UNORDERED_LIST_ITEM: &'static str = "unordered-list-item";
const ORDERED_LIST_ITEM: &'static str = "ordered-list-item";
const BLOCKQUOTE: &'static str = "blockquote";
const ATOMIC: &'static str = "atomic";
const CODE_BLOCK: &'static str = "code-block";
const UNSTYLED: &'static str = "unstyled";

/// The list of [default valid block types](https://draftjs.org/docs/advanced-topics-custom-block-render-map#draft-default-block-render-map),
/// according to the [`DefaultDraftBlockRenderMap`](https://github.com/facebook/draft-js/blob/main/src/model/immutable/DefaultDraftBlockRenderMap.js)
#[derive(Debug, Clone, PartialEq)]
pub enum RteBlockType {
    HeaderOne,
    HeaderTwo,
    HeaderThree,
    HeaderFour,
    HeaderFive,
    HeaderSix,
    Section,
    Article,
    UnorderedListItem,
    OrderedListItem,
    Blockquote,
    Atomic,
    CodeBlock,
    Unstyled,
    Custom(String),
}

impl RteBlockType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for RteBlockType {
    fn as_ref(&self) -> &str {
        match self {
            Self::HeaderOne => HEADER_ONE,
            Self::HeaderTwo => HEADER_TWO,
            Self::HeaderThree => HEADER_THREE,
            Self::HeaderFour => HEADER_FOUR,
            Self::HeaderFive => HEADER_FIVE,
            Self::HeaderSix => HEADER_SIX,
            Self::Section => SECTION,
            Self::Article => ARTICLE,
            Self::UnorderedListItem => UNORDERED_LIST_ITEM,
            Self::OrderedListItem => ORDERED_LIST_ITEM,
            Self::Blockquote => BLOCKQUOTE,
            Self::Atomic => ATOMIC,
            Self::CodeBlock => CODE_BLOCK,
            Self::Unstyled => UNSTYLED,
            Self::Custom(s) => &s,
        }
    }
}

impl Deref for RteBlockType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for RteBlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for RteBlockType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for RteBlockType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            HEADER_ONE => Ok(Self::HeaderOne),
            HEADER_TWO => Ok(Self::HeaderTwo),
            HEADER_THREE => Ok(Self::HeaderThree),
            HEADER_FOUR => Ok(Self::HeaderFour),
            HEADER_FIVE => Ok(Self::HeaderFive),
            HEADER_SIX => Ok(Self::HeaderSix),
            SECTION => Ok(Self::Section),
            ARTICLE => Ok(Self::Article),
            UNORDERED_LIST_ITEM => Ok(Self::UnorderedListItem),
            ORDERED_LIST_ITEM => Ok(Self::OrderedListItem),
            BLOCKQUOTE => Ok(Self::Blockquote),
            ATOMIC => Ok(Self::Atomic),
            CODE_BLOCK => Ok(Self::CodeBlock),
            UNSTYLED => Ok(Self::Unstyled),
            _ => Ok(Self::Custom(s.to_owned())),
        }
    }
}

impl Serialize for RteBlockType {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RteBlockType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
