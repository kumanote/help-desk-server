use crate::schema::faq_settings;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_settings)]
pub struct FaqSettings {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_settings)]
pub struct NewFaqSettings<'a> {
    pub id: &'a str,
    pub data: serde_json::Value,
}
