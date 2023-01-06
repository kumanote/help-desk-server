use crate::schema::agent_roles;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = agent_roles)]
pub struct AgentRole {
    pub agent_id: String,
    pub role_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = agent_roles)]
pub struct NewAgentRole<'a> {
    pub agent_id: &'a str,
    pub role_id: &'a str,
}
