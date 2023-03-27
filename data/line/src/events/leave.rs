use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
