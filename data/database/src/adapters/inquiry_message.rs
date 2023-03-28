use crate::entities::{InquiryMessage, NewInquiryMessage};
use crate::schema::inquiry_messages;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryMessage) -> Result<usize> {
    diesel::insert_into(inquiry_messages::table)
        .values(&entity)
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

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryMessage>> {
    inquiry_messages::table
        .filter(inquiry_messages::id.eq_any(ids))
        .order(inquiry_messages::id.desc())
        .load::<InquiryMessage>(conn)
        .map_err(Into::into)
}
