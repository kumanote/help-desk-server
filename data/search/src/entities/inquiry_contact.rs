use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InquiryContact {
    pub id: String,
    pub details: Vec<InquiryContactDetailItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InquiryContactDetailItem {
    #[serde(rename = "line")]
    Line(InquiryContactLineDetail),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InquiryContactLineDetail {
    pub display_name: String,
}
