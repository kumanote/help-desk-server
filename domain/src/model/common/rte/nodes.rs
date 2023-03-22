mod code;
mod heading;
mod image;
mod linebreak;
mod link;
mod list;
mod paragraph;
mod quote;
mod text;

pub use code::*;
pub use heading::*;
pub use image::*;
pub use linebreak::*;
pub use link::*;
pub use list::*;
pub use paragraph::*;
pub use quote::*;
pub use text::*;

use serde::{Deserialize, Serialize};

use super::{RteDirection, RteElementFormatType, RteIndent, RteVersion};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteRootNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    /// This value must be "root"
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: RteVersion,
}

impl RteTextsHolder for RteRootNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}

/// The Lexical ElementNode Definitions
/// * paragraph
/// * heading
/// * list
/// * lisitem
/// * quote
/// * image
/// * code
/// * code-highlight
/// * link
/// * text
/// * linebreak
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RteChildNode {
    #[serde(rename = "paragraph")]
    Paragraph(RteParagraphNode),
    #[serde(rename = "heading")]
    Heading(RteHeadingNode),
    #[serde(rename = "list")]
    List(RteListNode),
    #[serde(rename = "listitem")]
    ListItem(RteListItemNode),
    #[serde(rename = "quote")]
    Quote(RteQuoteNode),
    #[serde(rename = "image")]
    Image(RteImageNode),
    #[serde(rename = "code")]
    Code(RteCodeNode),
    #[serde(rename = "code-highlight")]
    CodeHighlight(RteCodeHighlightNode),
    #[serde(rename = "link")]
    Link(RteLinkNode),
    #[serde(rename = "text")]
    Text(RteTextNode),
    #[serde(rename = "linebreak")]
    Linebreak(RteLinebreakNode),
}

impl RteTextsHolder for RteChildNode {
    fn texts(&self) -> Vec<&str> {
        match self {
            Self::Paragraph(node) => node.texts(),
            Self::Heading(node) => node.texts(),
            Self::List(node) => node.texts(),
            Self::ListItem(node) => node.texts(),
            Self::Quote(node) => node.texts(),
            Self::Image(node) => node.texts(),
            Self::Code(node) => node.texts(),
            Self::CodeHighlight(node) => node.texts(),
            Self::Link(node) => node.texts(),
            Self::Text(node) => node.texts(),
            Self::Linebreak(node) => node.texts(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteElementNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
}

impl RteTextsHolder for RteElementNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}

pub trait RteTextsHolder {
    fn texts(&self) -> Vec<&str>;
}
