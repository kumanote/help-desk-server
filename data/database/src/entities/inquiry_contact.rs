use crate::schema::inquiry_contacts;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_contacts)]
pub struct InquiryContact {
    pub id: String,
    pub details: serde_json::Value,
    pub memo: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_contacts)]
pub struct NewInquiryContact<'a> {
    pub id: &'a str,
    pub details: serde_json::Value,
    pub memo: Option<&'a str>,
    pub created_at: NaiveDateTime,
}
