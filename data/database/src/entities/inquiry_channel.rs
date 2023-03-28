use crate::schema::inquiry_channels;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_channels)]
pub struct InquiryChannel {
    pub id: String,
    pub inquiry_channel_type: String,
    pub inquiry_channel_type_id: String,
    pub details: serde_json::Value,
    pub is_active: bool,
    pub activated_at: NaiveDateTime,
    pub deactivated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_channels)]
pub struct NewInquiryChannel<'a> {
    pub id: &'a str,
    pub inquiry_channel_type: &'a str,
    pub inquiry_channel_type_id: &'a str,
    pub details: serde_json::Value,
    pub is_active: bool,
    pub activated_at: NaiveDateTime,
    pub deactivated_at: Option<NaiveDateTime>,
}
