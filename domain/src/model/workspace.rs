use crate::model::{WorkspaceId, WorkspaceName};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: WorkspaceName,
    pub created_at: NaiveDateTime,
}

impl<'a> Into<database::entities::NewWorkspace<'a>> for &'a Workspace {
    fn into(self) -> database::entities::NewWorkspace<'a> {
        database::entities::NewWorkspace {
            id: &self.id,
            name: &self.name,
            created_at: &self.created_at,
        }
    }
}

impl From<database::entities::Workspace> for Workspace {
    fn from(value: database::entities::Workspace) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            created_at: value.created_at,
        }
    }
}
