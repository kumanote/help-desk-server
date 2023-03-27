use crate::entities::{InquirySettings, NewInquirySettings};
use crate::schema::inquiry_settings;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquirySettings) -> Result<usize> {
    diesel::insert_into(inquiry_settings::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update(conn: &mut DbConnection, data: &serde_json::Value, id: &str) -> Result<usize> {
    diesel::update(inquiry_settings::dsl::inquiry_settings.find(id))
        .set(inquiry_settings::data.eq(data))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get(conn: &mut DbConnection) -> Result<Option<InquirySettings>> {
    let result = inquiry_settings::table.first::<InquirySettings>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}
