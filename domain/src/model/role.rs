use super::{RoleId, RoleName, Scope};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    pub scopes: HashSet<Scope>,
}

impl Role {
    pub fn has_scope(&self, scope: &Scope) -> bool {
        self.scopes.contains(scope)
    }

    pub fn as_new_role_scope_entities(&self) -> Vec<database::entities::NewRoleScope> {
        self.scopes
            .iter()
            .map(|scope| database::entities::NewRoleScope {
                role_id: &self.id,
                scope: scope.as_str(),
            })
            .collect()
    }
}

impl<'a> Into<database::entities::NewRole<'a>> for &'a Role {
    fn into(self) -> database::entities::NewRole<'a> {
        database::entities::NewRole {
            id: &self.id,
            name: &self.name,
        }
    }
}
