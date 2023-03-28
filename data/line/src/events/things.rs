use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThingsEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub things: Things,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Things {
    #[serde(flatten)]
    pub r#type: ThingsType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ThingsType {
    #[serde(rename = "link")]
    Link {
        #[serde(rename = "deviceId")]
        device_id: String,
    },
    #[serde(rename = "unlink")]
    Unlink {
        #[serde(rename = "deviceId")]
        device_id: String,
    },
    #[serde(rename = "scenarioResult")]
    ScenarioResult {
        #[serde(rename = "deviceId")]
        device_id: String,
        result: ThingsResult,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThingsResult {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub revision: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "resultCode")]
    pub result_code: String,
    #[serde(rename = "bleNotificationPayload")]
    pub ble_notification_payload: String,
    #[serde(rename = "actionResults")]
    pub action_results: Vec<ActionResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionResult {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: String,
}
