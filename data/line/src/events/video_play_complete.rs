use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoPlayCompleteEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    #[serde(rename = "videoPlayComplete")]
    pub video_play_complete: VideoPlayComplete,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoPlayComplete {
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}
