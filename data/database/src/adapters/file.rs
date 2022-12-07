use crate::entities::{File, NewFile};
use crate::schema::files;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFile) -> Result<usize> {
    diesel::insert_into(files::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<File>> {
    let result = files::table.find(id).first::<File>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<File>> {
    files::table
        .filter(files::id.eq_any(ids))
        .order(files::id.desc())
        .load::<File>(conn)
        .map_err(Into::into)
}
