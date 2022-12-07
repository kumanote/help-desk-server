use crate::model::{GroupDescription, GroupId, GroupName};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: GroupName,
    pub description: GroupDescription,
    pub created_at: NaiveDateTime,
}
