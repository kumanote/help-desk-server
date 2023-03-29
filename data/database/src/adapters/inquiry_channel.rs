use crate::entities::{InquiryChannel, NewInquiryChannel};
use crate::schema::inquiry_channels;
use crate::{DbConnection, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryChannel) -> Result<usize> {
    diesel::insert_into(inquiry_channels::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_is_canceled_and_canceled_at_by_id(
    conn: &mut DbConnection,
    is_active: bool,
    activated_at: NaiveDateTime,
    deactivated_at: Option<NaiveDateTime>,
    id: &str,
) -> Result<usize> {
    diesel::update(inquiry_channels::dsl::inquiry_channels.find(id))
        .set((
            inquiry_channels::is_active.eq(is_active),
            inquiry_channels::activated_at.eq(activated_at),
            inquiry_channels::deactivated_at.eq(deactivated_at),
        ))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<InquiryChannel>> {
    let result = inquiry_channels::table
        .find(id)
        .first::<InquiryChannel>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_inquiry_channel_type_and_id(
    conn: &mut DbConnection,
    inquiry_channel_type: &str,
    inquiry_channel_type_id: &str,
) -> Result<Option<InquiryChannel>> {
    let result = inquiry_channels::table
        .filter(inquiry_channels::inquiry_channel_type.eq(inquiry_channel_type))
        .filter(inquiry_channels::inquiry_channel_type_id.eq(inquiry_channel_type_id))
        .first::<InquiryChannel>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryChannel>> {
    inquiry_channels::table
        .filter(inquiry_channels::id.eq_any(ids))
        .order(inquiry_channels::id.desc())
        .load::<InquiryChannel>(conn)
        .map_err(Into::into)
}
