use crate::entities::{FaqCategoryContent, NewFaqCategoryContent};
use crate::schema::faq_category_contents;
use crate::{DbConnection, Result};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewFaqCategoryContent) -> Result<usize> {
    diesel::insert_into(faq_category_contents::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_pk(conn: &mut DbConnection, faq_category_id: &str, locale: &str) -> Result<usize> {
    diesel::delete(faq_category_contents::table.find((faq_category_id, locale)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_title_by_pk(
    conn: &mut DbConnection,
    title: &str,
    faq_category_id: &str,
    locale: &str,
) -> Result<usize> {
    diesel::update(
        faq_category_contents::dsl::faq_category_contents.find((faq_category_id, locale)),
    )
    .set(faq_category_contents::title.eq(title))
    .execute(conn)
    .map_err(Into::into)
}

pub fn get_by_pk(
    conn: &mut DbConnection,
    faq_category_id: &str,
    locale: &str,
) -> Result<Option<FaqCategoryContent>> {
    let result = faq_category_contents::table
        .find((faq_category_id, locale))
        .first::<FaqCategoryContent>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_list_by_faq_category_id(
    conn: &mut DbConnection,
    faq_category_id: &str,
) -> Result<Vec<FaqCategoryContent>> {
    faq_category_contents::table
        .filter(faq_category_contents::faq_category_id.eq(faq_category_id))
        .load::<FaqCategoryContent>(conn)
        .map_err(Into::into)
}

pub fn get_list_by_faq_category_ids(
    conn: &mut DbConnection,
    faq_category_ids: &Vec<&str>,
) -> Result<Vec<FaqCategoryContent>> {
    faq_category_contents::table
        .filter(faq_category_contents::faq_category_id.eq_any(faq_category_ids))
        .load::<FaqCategoryContent>(conn)
        .map_err(Into::into)
}
