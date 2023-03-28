use crate::schema::inquiry_settings;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_settings)]
pub struct InquirySettings {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_settings)]
pub struct NewInquirySettings<'a> {
    pub id: &'a str,
    pub data: serde_json::Value,
}
