use crate::entities::{InquiryMessage, NewInquiryMessage};
use crate::schema::inquiry_messages;
use crate::{DbConnection, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryMessage) -> Result<usize> {
    diesel::insert_into(inquiry_messages::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_is_canceled_and_canceled_at_by_id(
    conn: &mut DbConnection,
    is_canceled: bool,
    canceled_at: Option<NaiveDateTime>,
    id: &str,
) -> Result<usize> {
    diesel::update(inquiry_messages::dsl::inquiry_messages.find(id))
        .set((
            inquiry_messages::is_canceled.eq(is_canceled),
            inquiry_messages::canceled_at.eq(canceled_at),
        ))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<InquiryMessage>> {
    let result = inquiry_messages::table
        .find(id)
        .first::<InquiryMessage>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_inquiry_message_type_and_id(
    conn: &mut DbConnection,
    inquiry_message_type: &str,
    inquiry_message_type_id: &str,
) -> Result<Option<InquiryMessage>> {
    let result = inquiry_messages::table
        .filter(inquiry_messages::inquiry_message_type.eq(inquiry_message_type))
        .filter(inquiry_messages::inquiry_message_type_id.eq(inquiry_message_type_id))
        .first::<InquiryMessage>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryMessage>> {
    inquiry_messages::table
        .filter(inquiry_messages::id.eq_any(ids))
        .order(inquiry_messages::id.desc())
        .load::<InquiryMessage>(conn)
        .map_err(Into::into)
}
