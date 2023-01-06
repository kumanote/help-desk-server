use crate::schema::agents;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = agents)]
pub struct Agent {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
    pub name: String,
    pub locale: String,
    pub is_active: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = agents)]
pub struct NewAgent<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub hashed_password: &'a str,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_active: bool,
}
