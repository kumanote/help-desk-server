use crate::schema::inquiry_contact_channels;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = inquiry_contact_channels)]
pub struct InquiryContactChannel {
    pub inquiry_contact_id: String,
    pub inquiry_channel_id: String,
    pub display_order: u32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = inquiry_contact_channels)]
pub struct NewInquiryContactChannel<'a> {
    pub inquiry_contact_id: &'a str,
    pub inquiry_channel_id: &'a str,
    pub display_order: u32,
}
