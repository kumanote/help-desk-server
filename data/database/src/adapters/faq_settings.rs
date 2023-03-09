use crate::entities::{FaqSettings, NewFaqSettings};
use crate::schema::faq_settings;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqSettings) -> Result<usize> {
    diesel::insert_into(faq_settings::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_by_id(conn: &mut DbConnection, data: &serde_json::Value, id: &str) -> Result<usize> {
    diesel::update(faq_settings::dsl::faq_settings.find(id))
        .set(faq_settings::data.eq(data))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get(conn: &mut DbConnection) -> Result<Option<FaqSettings>> {
    let result = faq_settings::table.first::<FaqSettings>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}
