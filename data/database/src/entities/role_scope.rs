use crate::schema::role_scopes;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = role_scopes)]
pub struct RoleScope {
    pub role_id: String,
    pub scope: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = role_scopes)]
pub struct NewRoleScope<'a> {
    pub role_id: &'a str,
    pub scope: &'a str,
}
