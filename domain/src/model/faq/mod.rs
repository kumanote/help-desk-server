mod faq_category;
pub use faq_category::*;

mod faq_category_content;
pub use faq_category_content::*;

mod faq_category_title;
pub use faq_category_title::*;

mod faq_item;
pub use faq_item::*;

mod faq_item_content;
pub use faq_item_content::*;

mod faq_item_title;
pub use faq_item_title::*;

pub type FaqItemBody = crate::model::common::RteContentState;
