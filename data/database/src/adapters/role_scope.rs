use crate::entities::{NewRoleScope, RoleScope};
use crate::schema::role_scopes;
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
