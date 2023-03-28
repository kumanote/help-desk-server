use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineMessageDetails {
    pub message: line::events::messages::Message,
}
