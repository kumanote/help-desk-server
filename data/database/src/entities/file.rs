use crate::schema::files;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = files)]
pub struct File {
    pub id: String,
    pub stored_filename: String,
    pub original_filename: String,
    pub mime_type: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub id: &'a str,
    pub stored_filename: &'a str,
    pub original_filename: &'a str,
    pub mime_type: &'a str,
}
