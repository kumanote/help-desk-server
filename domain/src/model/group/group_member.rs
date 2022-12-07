use crate::model::{AgentId, GroupId, RoleId};

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub agent_id: AgentId,
    pub role_id: RoleId,
}
