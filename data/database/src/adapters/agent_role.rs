use crate::entities::{AgentRole, NewAgentRole};
use crate::schema::agent_roles;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub fn bulk_create(conn: &mut DbConnection, entities: Vec<NewAgentRole>) -> Result<usize> {
    diesel::insert_into(agent_roles::table)
        .values(&entities)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_agent_id(conn: &mut DbConnection, agent_id: &str) -> Result<usize> {
    diesel::delete(agent_roles::table.filter(agent_roles::agent_id.eq(agent_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_list_by_agent_id(conn: &mut DbConnection, agent_id: &str) -> Result<Vec<AgentRole>> {
    agent_roles::table
        .filter(agent_roles::agent_id.eq(agent_id))
        .load::<AgentRole>(conn)
        .map_err(Into::into)
}
