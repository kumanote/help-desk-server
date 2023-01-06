use crate::model::{GroupDescription, GroupId, GroupName};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: GroupName,
    pub description: GroupDescription,
    pub created_at: NaiveDateTime,
}

impl<'a> Into<database::entities::NewGroup<'a>> for &'a Group {
    fn into(self) -> database::entities::NewGroup<'a> {
        database::entities::NewGroup {
            id: &self.id,
            name: &self.name,
            description: self.description.as_deref(),
            created_at: &self.created_at,
        }
    }
}

impl From<database::entities::Group> for Group {
    fn from(value: database::entities::Group) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            description: value.description.into(),
            created_at: value.created_at,
        }
    }
}
