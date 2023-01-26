use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Workspace {
    pub name: String,
    pub created_at: i64,
}

impl From<model::Workspace> for Workspace {
    fn from(value: model::Workspace) -> Self {
        Self {
            name: value.name.into(),
            created_at: value.created_at.timestamp(),
        }
    }
}
