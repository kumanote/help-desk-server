use crate::entities::{InquiryThread, NewInquiryThread};
use crate::schema::inquiry_threads;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryThread) -> Result<usize> {
    diesel::insert_into(inquiry_threads::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<InquiryThread>> {
    let result = inquiry_threads::table.find(id).first::<InquiryThread>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<InquiryThread>> {
    inquiry_threads::table
        .filter(inquiry_threads::id.eq_any(ids))
        .order(inquiry_threads::id.desc())
        .load::<InquiryThread>(conn)
        .map_err(Into::into)
}
