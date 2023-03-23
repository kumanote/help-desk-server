use crate::entities::{FaqItem, NewFaqItem};
use crate::schema::faq_items;
use crate::{DbConnection, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqItem) -> Result<usize> {
    diesel::insert_into(faq_items::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_id(conn: &mut DbConnection, id: &str) -> Result<usize> {
    diesel::delete(faq_items::dsl::faq_items.find(id))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_slug_by_id(conn: &mut DbConnection, slug: &str, id: &str) -> Result<usize> {
    diesel::update(faq_items::dsl::faq_items.find(id))
        .set(faq_items::slug.eq(slug))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_published_by_id(
    conn: &mut DbConnection,
    is_published: bool,
    published_at: Option<NaiveDateTime>,
    last_updated_at: Option<NaiveDateTime>,
    id: &str,
) -> Result<usize> {
    diesel::update(faq_items::dsl::faq_items.find(id))
        .set((
            faq_items::is_published.eq(is_published),
            faq_items::published_at.eq(published_at),
            faq_items::last_updated_at.eq(last_updated_at),
        ))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<FaqItem>> {
    let result = faq_items::table.find(id).first::<FaqItem>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_slug(conn: &mut DbConnection, slug: &str) -> Result<Option<FaqItem>> {
    let result = faq_items::table
        .filter(faq_items::slug.eq(slug))
        .first::<FaqItem>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<FaqItem>> {
    faq_items::table
        .filter(faq_items::id.eq_any(ids))
        .load::<FaqItem>(conn)
        .map_err(Into::into)
}
