use crate::entities::{FaqCategory, NewFaqCategory};
use crate::schema::faq_categories;
use crate::{DbConnection, Result};
use diesel::dsl::max;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqCategory) -> Result<usize> {
    diesel::insert_into(faq_categories::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_slug_by_id(conn: &mut DbConnection, slug: &str, id: &str) -> Result<usize> {
    diesel::update(faq_categories::dsl::faq_categories.find(id))
        .set(faq_categories::slug.eq(slug))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_display_order_by_id(
    conn: &mut DbConnection,
    display_order: u32,
    id: &str,
) -> Result<usize> {
    diesel::update(faq_categories::dsl::faq_categories.find(id))
        .set(faq_categories::display_order.eq(display_order))
        .execute(conn)
        .map_err(Into::into)
}

pub fn increment_display_order_by_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
) -> Result<usize> {
    Ok(diesel::update(
        faq_categories::dsl::faq_categories
            .filter(faq_categories::display_order.ge(from_display_order))
            .filter(faq_categories::display_order.le(to_display_order)),
    )
    .set(faq_categories::display_order.eq(faq_categories::display_order + 1))
    .execute(conn)?)
}

pub fn decrement_display_order_by_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
) -> Result<usize> {
    Ok(diesel::update(
        faq_categories::dsl::faq_categories
            .filter(faq_categories::display_order.ge(from_display_order))
            .filter(faq_categories::display_order.le(to_display_order)),
    )
    .set(faq_categories::display_order.eq(faq_categories::display_order - 1))
    .execute(conn)?)
}

pub fn get_by_id(conn: &mut DbConnection, id: &str) -> Result<Option<FaqCategory>> {
    let result = faq_categories::table.find(id).first::<FaqCategory>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_by_slug(conn: &mut DbConnection, slug: &str) -> Result<Option<FaqCategory>> {
    let result = faq_categories::table
        .filter(faq_categories::slug.eq(slug))
        .first::<FaqCategory>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_max_display_order(conn: &mut DbConnection) -> Result<Option<u32>> {
    Ok(faq_categories::table
        .select(max(faq_categories::display_order))
        .first(conn)?)
}

pub fn get_list_by_ids(conn: &mut DbConnection, ids: &Vec<&str>) -> Result<Vec<FaqCategory>> {
    faq_categories::table
        .filter(faq_categories::id.eq_any(ids))
        .load::<FaqCategory>(conn)
        .map_err(Into::into)
}
