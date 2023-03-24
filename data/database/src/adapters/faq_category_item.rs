use crate::entities::{FaqCategoryItem, NewFaqCategoryItem};
use crate::schema::faq_category_items;
use crate::{DbConnection, Result};
use diesel::dsl::max;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqCategoryItem) -> Result<usize> {
    diesel::insert_into(faq_category_items::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_pk(
    conn: &mut DbConnection,
    faq_category_id: &str,
    faq_item_id: &str,
) -> Result<usize> {
    diesel::delete(faq_category_items::table.find((faq_category_id, faq_item_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_display_order_by_id(
    conn: &mut DbConnection,
    display_order: u32,
    faq_category_id: &str,
    faq_item_id: &str,
) -> Result<usize> {
    diesel::update(faq_category_items::dsl::faq_category_items.find((faq_category_id, faq_item_id)))
        .set(faq_category_items::display_order.eq(display_order))
        .execute(conn)
        .map_err(Into::into)
}

pub fn increment_display_order_by_faq_category_id_and_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
    faq_category_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        faq_category_items::dsl::faq_category_items
            .filter(faq_category_items::faq_category_id.ge(faq_category_id))
            .filter(faq_category_items::display_order.ge(from_display_order))
            .filter(faq_category_items::display_order.le(to_display_order)),
    )
    .set(faq_category_items::display_order.eq(faq_category_items::display_order + 1))
    .execute(conn)?)
}

pub fn decrement_display_order_by_faq_category_id_and_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
    faq_category_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        faq_category_items::dsl::faq_category_items
            .filter(faq_category_items::faq_category_id.ge(faq_category_id))
            .filter(faq_category_items::display_order.ge(from_display_order))
            .filter(faq_category_items::display_order.le(to_display_order)),
    )
    .set(faq_category_items::display_order.eq(faq_category_items::display_order - 1))
    .execute(conn)?)
}

pub fn decrement_display_order_by_faq_category_id_and_from_display_order(
    conn: &mut DbConnection,
    from_display_order: u32,
    faq_category_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        faq_category_items::dsl::faq_category_items
            .filter(faq_category_items::faq_category_id.ge(faq_category_id))
            .filter(faq_category_items::display_order.ge(from_display_order)),
    )
    .set(faq_category_items::display_order.eq(faq_category_items::display_order - 1))
    .execute(conn)?)
}

pub fn get_by_id(
    conn: &mut DbConnection,
    faq_category_id: &str,
    faq_item_id: &str,
) -> Result<Option<FaqCategoryItem>> {
    let result = faq_category_items::table
        .find((faq_category_id, faq_item_id))
        .first::<FaqCategoryItem>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_max_display_order_by_faq_category_id(
    conn: &mut DbConnection,
    faq_category_id: &str,
) -> Result<Option<u32>> {
    Ok(faq_category_items::table
        .filter(faq_category_items::faq_category_id.ge(faq_category_id))
        .select(max(faq_category_items::display_order))
        .first(conn)?)
}

pub fn get_list_by_faq_item_id(
    conn: &mut DbConnection,
    faq_item_id: &str,
) -> Result<Vec<FaqCategoryItem>> {
    faq_category_items::table
        .filter(faq_category_items::faq_item_id.eq(faq_item_id))
        .load::<FaqCategoryItem>(conn)
        .map_err(Into::into)
}
