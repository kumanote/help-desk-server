use crate::entities::{Group, NewGroup};
use crate::schema::groups;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewGroup) -> Result<usize> {
    diesel::insert_into(groups::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<Group>> {
    let result = groups::table.find(id).first::<Group>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<Group>> {
    groups::table
        .filter(groups::id.eq_any(ids))
        .order(groups::id.desc())
        .load::<Group>(conn)
        .map_err(Into::into)
}
