use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

const TEXT_PLAIN: &'static str = "text/plain";
const TEXT_CSV: &'static str = "text/csv";
const TEXT_TSV: &'static str = "text/tab-separated-values";
const APPLICATION_PDF: &'static str = "application/pdf";
const APPLICATION_XLSX: &'static str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_PPTX: &'static str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";
const APPLICATION_DOCX: &'static str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const IMAGE_JPEG: &'static str = "image/jpeg";
const IMAGE_PNG: &'static str = "image/png";
const IMAGE_GIF: &'static str = "image/gif";
const IMAGE_WEBP: &'static str = "image/webp";
const IMAGE_BMP: &'static str = "image/bmp";
const IMAGE_HEIF: &'static str = "image/heif";
const APPLICATION_OCTET_STREAM: &'static str = "application/octet-stream";

/// File types supported in this application.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MimeType {
    TextPlain,
    Csv,
    Tsv,
    Pdf,
    Xlsx,
    Pptx,
    Docx,
    Jpeg,
    Png,
    Gif,
    Webp,
    Bitmap,
    Heif,
    Unknown,
}

impl MimeType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn is_image(&self) -> bool {
        match self {
            Self::Jpeg => true,
            Self::Png => true,
            Self::Gif => true,
            Self::Webp => true,
            Self::Bitmap => true,
            Self::Heif => true,
            _ => false,
        }
    }

    pub fn as_extension(&self) -> &str {
        match self {
            Self::TextPlain => "txt",
            Self::Csv => "csv",
            Self::Tsv => "tsv",
            Self::Pdf => "pdf",
            Self::Xlsx => "xlsx",
            Self::Pptx => "pptx",
            Self::Docx => "docx",
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Webp => "webp",
            Self::Bitmap => "bmp",
            Self::Heif => "heif",
            Self::Unknown => "",
        }
    }
}

impl AsRef<str> for MimeType {
    fn as_ref(&self) -> &str {
        match self {
            Self::TextPlain => TEXT_PLAIN,
            Self::Csv => TEXT_CSV,
            Self::Tsv => TEXT_TSV,
            Self::Pdf => APPLICATION_PDF,
            Self::Xlsx => APPLICATION_XLSX,
            Self::Pptx => APPLICATION_PPTX,
            Self::Docx => APPLICATION_DOCX,
            Self::Jpeg => IMAGE_JPEG,
            Self::Png => IMAGE_PNG,
            Self::Gif => IMAGE_GIF,
            Self::Webp => IMAGE_WEBP,
            Self::Bitmap => IMAGE_BMP,
            Self::Heif => IMAGE_HEIF,
            Self::Unknown => APPLICATION_OCTET_STREAM,
        }
    }
}

impl Deref for MimeType {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for MimeType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<String> for MimeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            TEXT_PLAIN => Self::TextPlain,
            TEXT_CSV => Self::Csv,
            TEXT_TSV => Self::Tsv,
            APPLICATION_PDF => Self::Pdf,
            APPLICATION_XLSX => Self::Xlsx,
            APPLICATION_PPTX => Self::Pptx,
            APPLICATION_DOCX => Self::Docx,
            IMAGE_JPEG => Self::Jpeg,
            IMAGE_PNG => Self::Png,
            IMAGE_GIF => Self::Gif,
            IMAGE_WEBP => Self::Webp,
            IMAGE_BMP => Self::Bitmap,
            IMAGE_HEIF => Self::Heif,
            _ => Self::Unknown,
        }
    }
}

impl Serialize for MimeType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MimeType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Into::into)
    }
}
