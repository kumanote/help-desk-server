use crate::model::{FaqItemId, Slug};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct FaqItem {
    pub id: FaqItemId,
    pub slug: Slug,
    pub is_published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub last_updated_at: Option<NaiveDateTime>,
}

impl<'a> Into<database::entities::NewFaqItem<'a>> for &'a FaqItem {
    fn into(self) -> database::entities::NewFaqItem<'a> {
        database::entities::NewFaqItem {
            id: &self.id,
            slug: &self.slug,
            is_published: self.is_published,
            published_at: self.published_at,
            last_updated_at: self.last_updated_at,
        }
    }
}

impl From<database::entities::FaqItem> for FaqItem {
    fn from(value: database::entities::FaqItem) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            is_published: value.is_published,
            published_at: value.published_at,
            last_updated_at: value.last_updated_at,
        }
    }
}
