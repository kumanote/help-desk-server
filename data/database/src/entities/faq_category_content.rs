use crate::schema::faq_category_contents;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_category_contents)]
pub struct FaqCategoryContent {
    pub faq_category_id: String,
    pub locale: String,
    pub title: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_category_contents)]
pub struct NewFaqCategoryContent<'a> {
    pub faq_category_id: &'a str,
    pub locale: &'a str,
    pub title: &'a str,
}
