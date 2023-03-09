use crate::schema::faq_items;
use chrono::NaiveDateTime;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_items)]
pub struct FaqItem {
    pub id: String,
    pub slug: String,
    pub is_published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub last_updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_items)]
pub struct NewFaqItem<'a> {
    pub id: &'a str,
    pub slug: &'a str,
    pub is_published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub last_updated_at: Option<NaiveDateTime>,
}
