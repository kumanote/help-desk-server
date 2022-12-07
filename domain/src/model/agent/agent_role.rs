use crate::model::{AgentId, RoleId};

#[derive(Debug, Clone)]
pub struct AgentRole {
    pub agent_id: AgentId,
    pub role_id: RoleId,
}
