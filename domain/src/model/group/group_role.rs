use crate::model::{GroupId, Role};

#[derive(Debug, Clone)]
pub struct GroupRoles {
    pub group_id: GroupId,
    pub roles: Vec<Role>,
}

impl GroupRoles {
    pub fn new(group_id: GroupId) -> Self {
        Self {
            group_id,
            roles: Vec::new(),
        }
    }

    pub fn as_new_group_role_entities(&self) -> Vec<database::entities::NewGroupRole> {
        self.roles
            .iter()
            .map(|role| database::entities::NewGroupRole {
                group_id: &self.group_id,
                role_id: &role.id,
            })
            .collect()
    }

    pub fn add_role(&mut self, role: Role) -> bool {
        if self
            .roles
            .iter()
            .find(|attached| attached.id == role.id)
            .is_some()
        {
            self.roles.push(role);
            false
        } else {
            true
        }
    }
}
