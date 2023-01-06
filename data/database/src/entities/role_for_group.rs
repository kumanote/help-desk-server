use crate::schema::roles_for_group;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = roles_for_group)]
pub struct RoleForGroup {
    pub id: String,
    pub name: String,
    pub scope: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = roles_for_group)]
pub struct NewRoleForGroup<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub scope: &'a str,
}
