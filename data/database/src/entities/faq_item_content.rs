use crate::schema::faq_item_contents;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = faq_item_contents)]
pub struct FaqItemContent {
    pub faq_item_id: String,
    pub locale: String,
    pub title: String,
    pub body: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = faq_item_contents)]
pub struct NewFaqItemContent<'a> {
    pub faq_item_id: &'a str,
    pub locale: &'a str,
    pub title: &'a str,
    pub body: serde_json::Value,
}
