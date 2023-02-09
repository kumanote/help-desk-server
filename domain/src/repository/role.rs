use crate::model::{AgentId, AgentRoles, GroupId, GroupRoles, Role, RoleId, Scope};

pub trait RoleRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, role: &Role) -> Result<(), Self::Err>;
    fn get_by_id(&self, tx: &mut Self::Transaction, id: &RoleId)
        -> Result<Option<Role>, Self::Err>;
    fn get_agent_roles(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
    ) -> Result<AgentRoles, Self::Err>;
    fn update_agent_roles(
        &self,
        tx: &mut Self::Transaction,
        agent_roles: &AgentRoles,
    ) -> Result<(), Self::Err>;
    fn get_group_roles(
        &self,
        tx: &mut Self::Transaction,
        group_id: &GroupId,
    ) -> Result<GroupRoles, Self::Err>;
    fn update_group_roles(
        &self,
        tx: &mut Self::Transaction,
        group_roles: &GroupRoles,
    ) -> Result<(), Self::Err>;
    fn check_agent_has_required_scopes(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
        scopes: &Vec<Scope>,
    ) -> Result<bool, Self::Err>;
    fn get_all_authorized_scopes_by_agent(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
    ) -> Result<Vec<Scope>, Self::Err>;
}
