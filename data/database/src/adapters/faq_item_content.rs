use crate::entities::{FaqItemContent, NewFaqItemContent};
use crate::schema::faq_item_contents;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqItemContent) -> Result<usize> {
    diesel::insert_into(faq_item_contents::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_pk(conn: &mut DbConnection, faq_item_id: &str, locale: &str) -> Result<usize> {
    diesel::delete(faq_item_contents::table.find((faq_item_id, locale)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_faq_item_id(conn: &mut DbConnection, faq_item_id: &str) -> Result<usize> {
    diesel::delete(faq_item_contents::table.filter(faq_item_contents::faq_item_id.eq(faq_item_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_by_pk(
    conn: &mut DbConnection,
    title: &str,
    body: &serde_json::Value,
    faq_item_id: &str,
    locale: &str,
) -> Result<usize> {
    diesel::update(faq_item_contents::dsl::faq_item_contents.find((faq_item_id, locale)))
        .set((
            faq_item_contents::title.eq(title),
            faq_item_contents::body.eq(body),
        ))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_pk(
    conn: &mut DbConnection,
    faq_item_id: &str,
    locale: &str,
) -> Result<Option<FaqItemContent>> {
    let result = faq_item_contents::table
        .find((faq_item_id, locale))
        .first::<FaqItemContent>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_faq_item_id(
    conn: &mut DbConnection,
    faq_item_id: &str,
) -> Result<Vec<FaqItemContent>> {
    faq_item_contents::table
        .filter(faq_item_contents::faq_item_id.eq(faq_item_id))
        .load::<FaqItemContent>(conn)
        .map_err(Into::into)
}
