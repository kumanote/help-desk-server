use database::DbConnection;
use domain::model::{AgentId, AgentRoles, GroupId, GroupRoles, Role, RoleId, Scope};
use domain::repository::RoleRepository;

pub struct RoleRepositoryImpl {
    // TODO add cache layer
}

impl RoleRepository for RoleRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, role: &Role) -> Result<(), Self::Err> {
        database::adapters::role::create(tx, role.into())?;
        database::adapters::role_scope::bulk_create(tx, role.as_new_role_scope_entities())?;
        Ok(())
    }

    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &RoleId,
    ) -> Result<Option<Role>, Self::Err> {
        if let Some(role_entity) = database::adapters::role::get_by_id(tx, &id)? {
            let role_scope_entities = database::adapters::role_scope::get_list_by_role_id(tx, &id)?;
            let scopes = role_scope_entities
                .into_iter()
                .map(|role_scope| Scope::from(role_scope.scope))
                .collect();
            Ok(Some(Role {
                id: role_entity.id.into(),
                name: role_entity.name.into(),
                scopes,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_agent_roles(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
    ) -> Result<AgentRoles, Self::Err> {
        let agent_role_entities =
            database::adapters::agent_role::get_list_by_agent_id(tx, &agent_id)?;
        let role_ids: Vec<&str> = agent_role_entities
            .iter()
            .map(|agent_role_entity| agent_role_entity.role_id.as_str())
            .collect();
        let role_entities = database::adapters::role::get_list_by_ids(tx, &role_ids)?;
        let role_ids: Vec<&str> = role_entities.iter().map(|role| role.id.as_str()).collect();
        let role_scope_entities =
            database::adapters::role_scope::get_list_by_role_ids(tx, &role_ids)?;
        let roles = role_entities
            .into_iter()
            .map(|role_entity| {
                let scopes = role_scope_entities
                    .iter()
                    .filter(|role_scope_entity| {
                        role_scope_entity.role_id.as_str() == role_entity.id.as_str()
                    })
                    .map(|role_scope_entity| Scope::from(role_scope_entity.scope.clone()))
                    .collect();
                Role {
                    id: role_entity.id.into(),
                    name: role_entity.name.into(),
                    scopes,
                }
            })
            .collect();
        Ok(AgentRoles {
            agent_id: agent_id.clone(),
            roles,
        })
    }

    fn update_agent_roles(
        &self,
        tx: &mut Self::Transaction,
        agent_roles: &AgentRoles,
    ) -> Result<(), Self::Err> {
        database::adapters::agent_role::delete_by_agent_id(tx, &agent_roles.agent_id)?;
        database::adapters::agent_role::bulk_create(tx, agent_roles.as_new_agent_role_entities())?;
        // TODO clear cache
        Ok(())
    }

    fn get_group_roles(
        &self,
        tx: &mut Self::Transaction,
        group_id: &GroupId,
    ) -> Result<GroupRoles, Self::Err> {
        let group_role_entities =
            database::adapters::group_role::get_list_by_group_id(tx, &group_id)?;
        let role_ids: Vec<&str> = group_role_entities
            .iter()
            .map(|group_role_entity| group_role_entity.role_id.as_str())
            .collect();
        let role_entities = database::adapters::role::get_list_by_ids(tx, &role_ids)?;
        let role_ids: Vec<&str> = role_entities.iter().map(|role| role.id.as_str()).collect();
        let role_scope_entities =
            database::adapters::role_scope::get_list_by_role_ids(tx, &role_ids)?;
        let roles = role_entities
            .into_iter()
            .map(|role_entity| {
                let scopes = role_scope_entities
                    .iter()
                    .filter(|role_scope_entity| {
                        role_scope_entity.role_id.as_str() == role_entity.id.as_str()
                    })
                    .map(|role_scope_entity| Scope::from(role_scope_entity.scope.clone()))
                    .collect();
                Role {
                    id: role_entity.id.into(),
                    name: role_entity.name.into(),
                    scopes,
                }
            })
            .collect();
        Ok(GroupRoles {
            group_id: group_id.clone(),
            roles,
        })
    }

    fn update_group_roles(
        &self,
        tx: &mut Self::Transaction,
        group_roles: &GroupRoles,
    ) -> Result<(), Self::Err> {
        database::adapters::group_role::delete_by_group_id(tx, &group_roles.group_id)?;
        database::adapters::group_role::bulk_create(tx, group_roles.as_new_group_role_entities())?;
        // TODO clear cache
        Ok(())
    }
}
