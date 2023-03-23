use crate::model::{FaqCategoryItemWithCategory, FaqItemContent, FaqItemId, Slug};
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

#[derive(Debug, Clone)]
pub struct FaqItemWithContentsAndCategories {
    pub id: FaqItemId,
    pub slug: Slug,
    pub is_published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub last_updated_at: Option<NaiveDateTime>,
    pub contents: Vec<FaqItemContent>,
    pub categories: Vec<FaqCategoryItemWithCategory>,
}

impl Into<queue::entities::FaqItemWithContentsAndCategories> for FaqItemWithContentsAndCategories {
    fn into(self) -> queue::entities::FaqItemWithContentsAndCategories {
        queue::entities::FaqItemWithContentsAndCategories {
            id: self.id.into(),
            slug: self.slug.into(),
            is_published: self.is_published,
            published_at: self.published_at,
            last_updated_at: self.last_updated_at,
            contents: self.contents.into_iter().map(Into::into).collect(),
            categories: self.categories.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<queue::entities::FaqItemWithContentsAndCategories> for FaqItemWithContentsAndCategories {
    fn from(value: queue::entities::FaqItemWithContentsAndCategories) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            is_published: value.is_published,
            published_at: value.published_at,
            last_updated_at: value.last_updated_at,
            contents: value.contents.into_iter().map(Into::into).collect(),
            categories: value.categories.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<search::entities::FaqItem> for &FaqItemWithContentsAndCategories {
    fn into(self) -> search::entities::FaqItem {
        let mut categories = vec![];
        for category_with_contents in &self.categories {
            for content in &category_with_contents.category.contents {
                categories.push(search::entities::FaqItemCategory {
                    locale: content.locale.to_string(),
                    title: content.title.to_string(),
                })
            }
        }
        search::entities::FaqItem {
            id: self.id.to_string(),
            contents: self
                .contents
                .iter()
                .map(|content| search::entities::FaqItemContent {
                    locale: content.locale.to_string(),
                    title: content.title.to_string(),
                    body: content.body.text(),
                })
                .collect(),
            categories,
        }
    }
}

impl
    From<(
        FaqItem,
        Vec<FaqItemContent>,
        Vec<FaqCategoryItemWithCategory>,
    )> for FaqItemWithContentsAndCategories
{
    fn from(
        (item, contents, categories): (
            FaqItem,
            Vec<FaqItemContent>,
            Vec<FaqCategoryItemWithCategory>,
        ),
    ) -> Self {
        Self {
            id: item.id,
            slug: item.slug,
            is_published: item.is_published,
            published_at: item.published_at,
            last_updated_at: item.last_updated_at,
            contents,
            categories,
        }
    }
}

pub type SearchedFaqItem = search::entities::FaqItem;
