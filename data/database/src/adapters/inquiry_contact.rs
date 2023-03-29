use crate::entities::{InquiryContact, NewInquiryContact};
use crate::schema::inquiry_contacts;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryContact) -> Result<usize> {
    diesel::insert_into(inquiry_contacts::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<InquiryContact>> {
    let result = inquiry_contacts::table
        .find(id)
        .first::<InquiryContact>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_line_user_id(
    conn: &mut DbConnection,
    line_user_id: &str,
) -> Result<Option<InquiryContact>> {
    let result = inquiry_contacts::table
        .filter(inquiry_contacts::line_user_id.eq(line_user_id))
        .first::<InquiryContact>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryContact>> {
    inquiry_contacts::table
        .filter(inquiry_contacts::id.eq_any(ids))
        .order(inquiry_contacts::id.desc())
        .load::<InquiryContact>(conn)
        .map_err(Into::into)
}
