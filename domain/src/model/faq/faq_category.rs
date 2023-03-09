use crate::model::{FaqCategoryId, Slug};

#[derive(Debug, Clone)]
pub struct FaqCategory {
    pub id: FaqCategoryId,
    pub slug: Slug,
    pub display_order: u32,
}

impl<'a> Into<database::entities::NewFaqCategory<'a>> for &'a FaqCategory {
    fn into(self) -> database::entities::NewFaqCategory<'a> {
        database::entities::NewFaqCategory {
            id: &self.id,
            slug: &self.slug,
            display_order: self.display_order,
        }
    }
}

impl From<database::entities::FaqCategory> for FaqCategory {
    fn from(value: database::entities::FaqCategory) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            display_order: value.display_order,
        }
    }
}
