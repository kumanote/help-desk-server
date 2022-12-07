use super::{RoleId, RoleName, Scope};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    pub scopes: HashSet<Scope>,
}
