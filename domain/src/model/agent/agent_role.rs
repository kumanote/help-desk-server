use crate::model::{AgentId, Role};

#[derive(Debug, Clone)]
pub struct AgentRoles {
    pub agent_id: AgentId,
    pub roles: Vec<Role>,
}

impl AgentRoles {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            roles: Vec::new(),
        }
    }

    pub fn as_new_agent_role_entities(&self) -> Vec<database::entities::NewAgentRole> {
        self.roles
            .iter()
            .map(|role| database::entities::NewAgentRole {
                agent_id: &self.agent_id,
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
