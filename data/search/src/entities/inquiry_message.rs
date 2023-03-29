use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InquiryMessage {
    pub id: String,
    #[serde(flatten)]
    pub details: InquiryMessageDetailItem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InquiryMessageDetailItem {
    #[serde(rename = "line")]
    Line(InquiryMessageLineDetail),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InquiryMessageLineDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
