use super::{RoleForGroupId, RoleForGroupName, ScopeForGroup};

#[derive(Debug, Clone)]
pub struct RoleForGroup {
    pub id: RoleForGroupId,
    pub name: RoleForGroupName,
    pub scope: ScopeForGroup,
}

impl<'a> Into<database::entities::NewRoleForGroup<'a>> for &'a RoleForGroup {
    fn into(self) -> database::entities::NewRoleForGroup<'a> {
        database::entities::NewRoleForGroup {
            id: &self.id,
            name: &self.name,
            scope: &self.scope,
        }
    }
}

impl From<database::entities::RoleForGroup> for RoleForGroup {
    fn from(value: database::entities::RoleForGroup) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            scope: value.scope.into(),
        }
    }
}
