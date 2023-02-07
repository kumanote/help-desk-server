use crate::entities::{Agent, NewAgent};
use crate::schema::agents;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewAgent) -> Result<usize> {
    diesel::insert_into(agents::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<Agent>> {
    let result = agents::table.find(id).first::<Agent>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_email(conn: &mut DbConnection, email: &str) -> Result<Option<Agent>> {
    let result = agents::table
        .filter(agents::email.eq(email))
        .first::<Agent>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<Agent>> {
    agents::table
        .filter(agents::id.eq_any(ids))
        .order(agents::id.desc())
        .load::<Agent>(conn)
        .map_err(Into::into)
}
