use cache::CacheConnectionPool;
use chrono::Duration;
use database::DbConnection;
use domain::model::{AgentId, AgentRoles, GroupId, GroupRoles, Role, RoleId, Scope};
use domain::repository::RoleRepository;

const DEFAULT_AGENT_SCOPES_CACHE_MINUTES: i64 = 30;

pub struct RoleRepositoryImpl {
    cache_connection_pool: CacheConnectionPool,
    agent_scopes_cache_ttl: Duration,
}

impl RoleRepositoryImpl {
    pub fn new(cache_connection_pool: CacheConnectionPool) -> Self {
        Self {
            cache_connection_pool,
            agent_scopes_cache_ttl: Duration::minutes(DEFAULT_AGENT_SCOPES_CACHE_MINUTES),
        }
    }

    pub fn new_with_ttl(
        cache_connection_pool: CacheConnectionPool,
        agent_scopes_cache_ttl: Duration,
    ) -> Self {
        Self {
            cache_connection_pool,
            agent_scopes_cache_ttl,
        }
    }
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
        // clear cache
        let mut cache_conn = self.cache_connection_pool.get()?;
        cache::adapters::auth_agent_scopes::delete_by_agent_id(
            &mut cache_conn,
            &agent_roles.agent_id,
        )?;
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
        // clear all group member agent roles cache
        let mut cache_conn = self.cache_connection_pool.get()?;
        let mut last_agent_id = None;
        let batch_size = 1000;
        loop {
            let group_member_entities = database::adapters::group_member::get_list_by_group_id(
                tx,
                &group_roles.group_id,
                last_agent_id.as_deref(),
                batch_size,
            )?;
            for group_member in &group_member_entities {
                last_agent_id = Some(group_member.agent_id.to_owned());
                cache::adapters::auth_agent_scopes::delete_by_agent_id(
                    &mut cache_conn,
                    &group_member.agent_id,
                )?;
            }
            let fetch_size = group_member_entities.len() as i64;
            if fetch_size < batch_size {
                break;
            }
        }
        Ok(())
    }

    fn check_agent_has_required_scopes(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
        scopes: &Vec<Scope>,
    ) -> Result<bool, Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        if !cache::adapters::auth_agent_scopes::has_scopes(&mut cache_conn, &agent_id)? {
            // query all roles that user has and add them to cache.
            let all_role_scopes =
                database::adapters::role_scope::get_list_by_agent_id(tx, &agent_id)?;
            let scopes_to_be_set = all_role_scopes
                .iter()
                .map(|role_scope_entity| role_scope_entity.scope.as_str())
                .collect();
            cache::adapters::auth_agent_scopes::add_scopes_to_agent(
                &mut cache_conn,
                &agent_id,
                scopes_to_be_set,
                self.agent_scopes_cache_ttl,
            )?;
        }
        // if cache remains, check by scopes cache
        let scopes = scopes.into_iter().map(AsRef::as_ref).collect();
        cache::adapters::auth_agent_scopes::check_agent_has_all_scopes(
            &mut cache_conn,
            &agent_id,
            scopes,
        )
        .map_err(Into::into)
    }

    fn get_all_authorized_scopes_by_agent(
        &self,
        tx: &mut Self::Transaction,
        agent_id: &AgentId,
    ) -> Result<Vec<Scope>, Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        let cached_scopes =
            cache::adapters::auth_agent_scopes::get_all_scopes(&mut cache_conn, &agent_id)?;
        if cached_scopes.is_empty() {
            // query all roles that user has and add them to cache.
            let all_role_scopes =
                database::adapters::role_scope::get_list_by_agent_id(tx, &agent_id)?;
            let scopes: Vec<Scope> = all_role_scopes
                .into_iter()
                .map(|role_scope_entity| Scope::from(role_scope_entity.scope))
                .collect();
            // save cache
            cache::adapters::auth_agent_scopes::add_scopes_to_agent(
                &mut cache_conn,
                &agent_id,
                scopes.iter().map(|s| s.as_ref()).collect(),
                self.agent_scopes_cache_ttl,
            )?;
            Ok(scopes)
        } else {
            Ok(cached_scopes.into_iter().map(Into::into).collect())
        }
    }
}
