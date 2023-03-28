use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostBackEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub postback: PostBack,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostBack {
    pub data: String,
    pub params: Option<Params>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Params {
    pub date: Option<String>,
    pub time: Option<String>,
    pub datetime: Option<String>,
}
