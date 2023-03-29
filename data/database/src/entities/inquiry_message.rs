use crate::schema::inquiry_messages;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_messages)]
pub struct InquiryMessage {
    pub id: String,
    pub inquiry_thread_id: String,
    pub reply_inquiry_message_id: Option<String>,
    pub inquiry_message_type: String,
    pub inquiry_message_type_id: String,
    pub details: serde_json::Value,
    pub speaker_type: String,
    pub inquiry_contact_id: Option<String>,
    pub agent_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_messages)]
pub struct NewInquiryMessage<'a> {
    pub id: &'a str,
    pub inquiry_thread_id: &'a str,
    pub reply_inquiry_message_id: Option<&'a str>,
    pub inquiry_message_type: &'a str,
    pub inquiry_message_type_id: &'a str,
    pub details: serde_json::Value,
    pub speaker_type: &'a str,
    pub inquiry_contact_id: Option<&'a str>,
    pub agent_id: Option<&'a str>,
    pub created_at: NaiveDateTime,
}
