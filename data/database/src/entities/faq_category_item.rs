use crate::schema::faq_category_items;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_category_items)]
pub struct FaqCategoryItem {
    pub faq_category_id: String,
    pub faq_item_id: String,
    pub display_order: u32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_category_items)]
pub struct NewFaqCategoryItem<'a> {
    pub faq_category_id: &'a str,
    pub faq_item_id: &'a str,
    pub display_order: u32,
}
