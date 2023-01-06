use crate::entities::{NewWorkspace, Workspace};
use crate::schema::workspaces;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewWorkspace) -> Result<usize> {
    diesel::insert_into(workspaces::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update(conn: &mut DbConnection, name: &str, id: &str) -> Result<usize> {
    diesel::update(workspaces::dsl::workspaces.find(id))
        .set(workspaces::name.eq(name))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get(conn: &mut DbConnection) -> Result<Option<Workspace>> {
    let result = workspaces::table.first::<Workspace>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}
