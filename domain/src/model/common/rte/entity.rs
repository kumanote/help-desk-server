use super::{RteEntityMutability, RteEntityType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteEntity {
    #[serde(rename = "type")]
    pub r#type: RteEntityType,
    pub mutability: RteEntityMutability,
    pub data: serde_json::Map<String, serde_json::Value>,
}
