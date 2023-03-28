use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
