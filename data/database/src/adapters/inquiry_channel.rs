use crate::entities::{InquiryChannel, NewInquiryChannel};
use crate::schema::inquiry_channels;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryChannel) -> Result<usize> {
    diesel::insert_into(inquiry_channels::table)
        .values(&entity)
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

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryChannel>> {
    inquiry_channels::table
        .filter(inquiry_channels::id.eq_any(ids))
        .order(inquiry_channels::id.desc())
        .load::<InquiryChannel>(conn)
        .map_err(Into::into)
}
