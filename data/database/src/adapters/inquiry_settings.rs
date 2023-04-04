use crate::entities::{InquirySettings, NewInquirySettings};
use crate::schema::inquiry_settings;
use crate::{DbConnection, Result};
use diesel::result::Error;
use diesel::RunQueryDsl;

pub fn upsert(conn: &mut DbConnection, entity: NewInquirySettings) -> Result<usize> {
    diesel::replace_into(inquiry_settings::table)
        .values(&entity)
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
