use crate::schema::faq_categories;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_categories)]
pub struct FaqCategory {
    pub id: String,
    pub slug: String,
    pub display_order: u32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_categories)]
pub struct NewFaqCategory<'a> {
    pub id: &'a str,
    pub slug: &'a str,
    pub display_order: u32,
}
