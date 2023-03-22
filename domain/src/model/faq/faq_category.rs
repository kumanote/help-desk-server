use crate::model::{FaqCategoryContent, FaqCategoryId, Slug};

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

#[derive(Debug, Clone)]
pub struct FaqCategoryWithContents {
    pub id: FaqCategoryId,
    pub slug: Slug,
    pub display_order: u32,
    pub contents: Vec<FaqCategoryContent>,
}

impl From<(FaqCategory, Vec<FaqCategoryContent>)> for FaqCategoryWithContents {
    fn from((category, contents): (FaqCategory, Vec<FaqCategoryContent>)) -> Self {
        Self {
            id: category.id,
            slug: category.slug,
            display_order: category.display_order,
            contents,
        }
    }
}

impl Into<queue::entities::FaqCategoryWithContents> for FaqCategoryWithContents {
    fn into(self) -> queue::entities::FaqCategoryWithContents {
        queue::entities::FaqCategoryWithContents {
            id: self.id.into(),
            slug: self.slug.into(),
            display_order: self.display_order,
            contents: self.contents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<queue::entities::FaqCategoryWithContents> for FaqCategoryWithContents {
    fn from(value: queue::entities::FaqCategoryWithContents) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            display_order: value.display_order,
            contents: value.contents.into_iter().map(Into::into).collect(),
        }
    }
}
