use crate::schema::roles;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
