use crate::model::{Agent, GroupId, RoleForGroup};

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub agent: Agent,
    pub role: RoleForGroup,
}

impl GroupMember {
    pub fn new(group_id: GroupId, agent: Agent, role: RoleForGroup) -> Self {
        Self {
            group_id,
            agent,
            role,
        }
    }
}

impl<'a> Into<database::entities::NewGroupMember<'a>> for &'a GroupMember {
    fn into(self) -> database::entities::NewGroupMember<'a> {
        database::entities::NewGroupMember {
            group_id: &self.group_id,
            agent_id: &self.agent.id,
            role_id: &self.role.id,
        }
    }
}
