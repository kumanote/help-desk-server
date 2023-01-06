use crate::entities::{GroupRole, NewGroupRole};
use crate::schema::group_roles;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn bulk_create(conn: &mut DbConnection, entities: Vec<NewGroupRole>) -> Result<usize> {
    diesel::insert_into(group_roles::table)
        .values(&entities)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_group_id(conn: &mut DbConnection, group_id: &str) -> Result<usize> {
    diesel::delete(group_roles::table.filter(group_roles::group_id.eq(group_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_list_by_group_id(conn: &mut DbConnection, group_id: &str) -> Result<Vec<GroupRole>> {
    group_roles::table
        .filter(group_roles::group_id.eq(group_id))
        .load::<GroupRole>(conn)
        .map_err(Into::into)
}
