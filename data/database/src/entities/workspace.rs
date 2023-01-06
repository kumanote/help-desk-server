use crate::schema::workspaces;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = workspaces)]
pub struct NewWorkspace<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub created_at: &'a NaiveDateTime,
}
