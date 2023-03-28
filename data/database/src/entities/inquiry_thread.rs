use crate::schema::inquiry_threads;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_threads)]
pub struct InquiryThread {
    pub id: String,
    pub inquiry_channel_id: String,
    pub subject: String,
    pub inquiry_thread_type: String,
    pub inquiry_thread_type_id: String,
    pub details: serde_json::Value,
    pub status: String,
    pub assigned_agent_id: Option<String>,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_threads)]
pub struct NewInquiryThread<'a> {
    pub id: &'a str,
    pub inquiry_channel_id: &'a str,
    pub subject: &'a str,
    pub inquiry_thread_type: &'a str,
    pub inquiry_thread_type_id: &'a str,
    pub details: serde_json::Value,
    pub status: &'a str,
    pub assigned_agent_id: Option<&'a str>,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}
