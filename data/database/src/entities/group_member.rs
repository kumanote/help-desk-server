use crate::schema::group_members;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = group_members)]
pub struct GroupMember {
    pub group_id: String,
    pub agent_id: String,
    pub role_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = group_members)]
pub struct NewGroupMember<'a> {
    pub group_id: &'a str,
    pub agent_id: &'a str,
    pub role_id: &'a str,
}
