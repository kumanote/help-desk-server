use crate::entities::{NewRoleScope, RoleScope};
use crate::schema::{agent_roles, group_members, group_roles, role_scopes};
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn bulk_create(conn: &mut DbConnection, entities: Vec<NewRoleScope>) -> Result<usize> {
    diesel::insert_into(role_scopes::table)
        .values(&entities)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_role_id(conn: &mut DbConnection, role_id: &str) -> Result<usize> {
    diesel::delete(role_scopes::table.filter(role_scopes::role_id.eq(role_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_list_by_role_id(conn: &mut DbConnection, role_id: &str) -> Result<Vec<RoleScope>> {
    role_scopes::table
        .filter(role_scopes::role_id.eq(role_id))
        .load::<RoleScope>(conn)
        .map_err(Into::into)
}

pub fn get_list_by_role_ids(
    conn: &mut DbConnection,
    role_ids: &Vec<&str>,
) -> Result<Vec<RoleScope>> {
    role_scopes::table
        .filter(role_scopes::role_id.eq_any(role_ids))
        .load::<RoleScope>(conn)
        .map_err(Into::into)
}

pub fn get_list_by_agent_id(conn: &mut DbConnection, agent_id: &str) -> Result<Vec<RoleScope>> {
    let agent_role_ids = agent_roles::table
        .filter(agent_roles::agent_id.eq(agent_id))
        .select(agent_roles::role_id);
    let joined_group_ids = group_members::table
        .filter(group_members::agent_id.eq(agent_id))
        .select(group_members::group_id);
    let group_role_ids = group_roles::table
        .filter(group_roles::group_id.eq_any(joined_group_ids))
        .select(group_roles::role_id);
    role_scopes::table
        .filter(
            role_scopes::role_id
                .eq_any(agent_role_ids)
                .or(role_scopes::role_id.eq_any(group_role_ids)),
        )
        .load::<RoleScope>(conn)
        .map_err(Into::into)
}
