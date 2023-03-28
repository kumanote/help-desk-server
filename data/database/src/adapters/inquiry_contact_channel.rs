use crate::entities::{InquiryContactChannel, NewInquiryContactChannel};
use crate::schema::inquiry_contact_channels;
use crate::{DbConnection, Result};
use diesel::dsl::max;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn create(conn: &mut DbConnection, entity: NewInquiryContactChannel) -> Result<usize> {
    diesel::insert_into(inquiry_contact_channels::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete_by_pk(
    conn: &mut DbConnection,
    inquiry_contact_id: &str,
    inquiry_channel_id: &str,
) -> Result<usize> {
    diesel::delete(inquiry_contact_channels::table.find((inquiry_contact_id, inquiry_channel_id)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn update_display_order_by_pk(
    conn: &mut DbConnection,
    display_order: u32,
    inquiry_contact_id: &str,
    inquiry_channel_id: &str,
) -> Result<usize> {
    diesel::update(
        inquiry_contact_channels::dsl::inquiry_contact_channels
            .find((inquiry_contact_id, inquiry_channel_id)),
    )
    .set(inquiry_contact_channels::display_order.eq(display_order))
    .execute(conn)
    .map_err(Into::into)
}

pub fn increment_display_order_by_inquiry_contact_id_and_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
    inquiry_contact_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        inquiry_contact_channels::dsl::inquiry_contact_channels
            .filter(inquiry_contact_channels::inquiry_contact_id.ge(inquiry_contact_id))
            .filter(inquiry_contact_channels::display_order.ge(from_display_order))
            .filter(inquiry_contact_channels::display_order.le(to_display_order)),
    )
    .set(inquiry_contact_channels::display_order.eq(inquiry_contact_channels::display_order + 1))
    .execute(conn)?)
}

pub fn decrement_display_order_by_inquiry_contact_id_and_range(
    conn: &mut DbConnection,
    from_display_order: u32,
    to_display_order: u32,
    inquiry_contact_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        inquiry_contact_channels::dsl::inquiry_contact_channels
            .filter(inquiry_contact_channels::inquiry_contact_id.ge(inquiry_contact_id))
            .filter(inquiry_contact_channels::display_order.ge(from_display_order))
            .filter(inquiry_contact_channels::display_order.le(to_display_order)),
    )
    .set(inquiry_contact_channels::display_order.eq(inquiry_contact_channels::display_order - 1))
    .execute(conn)?)
}

pub fn decrement_display_order_by_inquiry_contact_id_and_from_display_order(
    conn: &mut DbConnection,
    from_display_order: u32,
    inquiry_contact_id: &str,
) -> Result<usize> {
    Ok(diesel::update(
        inquiry_contact_channels::dsl::inquiry_contact_channels
            .filter(inquiry_contact_channels::inquiry_contact_id.ge(inquiry_contact_id))
            .filter(inquiry_contact_channels::display_order.ge(from_display_order)),
    )
    .set(inquiry_contact_channels::display_order.eq(inquiry_contact_channels::display_order - 1))
    .execute(conn)?)
}

pub fn get_by_pk(
    conn: &mut DbConnection,
    inquiry_contact_id: &str,
    inquiry_channel_id: &str,
) -> Result<Option<InquiryContactChannel>> {
    let result = inquiry_contact_channels::table
        .find((inquiry_contact_id, inquiry_channel_id))
        .first::<InquiryContactChannel>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn get_max_display_order_by_inquiry_contact_id(
    conn: &mut DbConnection,
    inquiry_contact_id: &str,
) -> Result<Option<u32>> {
    Ok(inquiry_contact_channels::table
        .filter(inquiry_contact_channels::inquiry_contact_id.ge(inquiry_contact_id))
        .select(max(inquiry_contact_channels::display_order))
        .first(conn)?)
}
