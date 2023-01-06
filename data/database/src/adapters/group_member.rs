use crate::entities::{GroupMember, NewGroupMember};
use crate::schema::group_members;
use crate::{DbConnection, Result};
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewGroupMember) -> Result<usize> {
    diesel::insert_into(group_members::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_pk(conn: &mut DbConnection, group_id: &str, agent_id: &str) -> Result<usize> {
    diesel::delete(group_members::table.find((group_id, agent_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_pk(
    conn: &mut DbConnection,
    group_id: &str,
    agent_id: &str,
) -> Result<Option<GroupMember>> {
    let result = group_members::table
        .find((group_id, agent_id))
        .first::<GroupMember>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}
