use crate::entities::{NewRoleForGroup, RoleForGroup};
use crate::schema::roles_for_group;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewRoleForGroup) -> Result<usize> {
    diesel::insert_into(roles_for_group::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<RoleForGroup>> {
    let result = roles_for_group::table.find(id).first::<RoleForGroup>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<RoleForGroup>> {
    roles_for_group::table
        .filter(roles_for_group::id.eq_any(ids))
        .order(roles_for_group::id.desc())
        .load::<RoleForGroup>(conn)
        .map_err(Into::into)
}
