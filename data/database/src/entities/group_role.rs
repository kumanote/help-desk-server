use crate::schema::group_roles;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = group_roles)]
pub struct GroupRole {
    pub group_id: String,
    pub role_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = group_roles)]
pub struct NewGroupRole<'a> {
    pub group_id: &'a str,
    pub role_id: &'a str,
}
