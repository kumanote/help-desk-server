use crate::entities::{NewRole, Role};
use crate::schema::roles;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewRole) -> Result<usize> {
    diesel::insert_into(roles::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<Role>> {
    let result = roles::table.find(id).first::<Role>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<Role>> {
    roles::table
        .filter(roles::id.eq_any(ids))
        .order(roles::id.desc())
        .load::<Role>(conn)
        .map_err(Into::into)
}
