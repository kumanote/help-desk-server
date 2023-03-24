use crate::{
    model::{
        FaqCategoryId, FaqCategoryItem, FaqCategoryItemWithCategory, FaqContentLocale, FaqItemBody,
        FaqItemContent, FaqItemId, FaqItemTitle, FaqItemWithContentsAndCategories, Slug,
    },
    repository::{FaqRepository, FaqSearchRepository, PublicFaqSearchRepository},
    Error, Result,
};
use chrono::Utc;
use std::str::FromStr;

pub struct UpdateFaqItemContent {
    pub locale: String,
    pub title: String,
    pub body: String,
}

pub struct UpdateFaqItemUseCaseInput {
    pub id: String,
    pub slug: String,
    pub is_published: bool,
    pub contents: Vec<UpdateFaqItemContent>,
    /// category ids
    pub categories: Vec<String>,
}

pub type UpdateFaqItemUseCaseOutput = FaqItemWithContentsAndCategories;

pub trait UpdateFaqItemUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    type PublicFaqSearchRepository: PublicFaqSearchRepository<Err = Error>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqItemUseCaseInput,
    ) -> Result<UpdateFaqItemUseCaseOutput>;
}

pub struct UpdateFaqItemUseCaseImpl<
    FR: FaqRepository<Err = Error>,
    FSR: FaqSearchRepository<Err = Error>,
    PFSR: PublicFaqSearchRepository<Err = Error>,
> {
    faq_repository: FR,
    faq_search_repository: FSR,
    public_faq_search_repository: PFSR,
}

impl<
        FR: FaqRepository<Err = Error>,
        FSR: FaqSearchRepository<Err = Error>,
        PFSR: PublicFaqSearchRepository<Err = Error>,
    > UpdateFaqItemUseCaseImpl<FR, FSR, PFSR>
{
    pub fn new(
        faq_repository: FR,
        faq_search_repository: FSR,
        public_faq_search_repository: PFSR,
    ) -> Self {
        Self {
            faq_repository,
            faq_search_repository,
            public_faq_search_repository,
        }
    }
}

impl<
        TX,
        FR: FaqRepository<Err = Error, Transaction = TX>,
        FSR: FaqSearchRepository<Err = Error>,
        PFSR: PublicFaqSearchRepository<Err = Error>,
    > UpdateFaqItemUseCase for UpdateFaqItemUseCaseImpl<FR, FSR, PFSR>
{
    type Transaction = TX;
    type FaqRepository = FR;
    type FaqSearchRepository = FSR;
    type PublicFaqSearchRepository = PFSR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqItemUseCaseInput,
    ) -> Result<UpdateFaqItemUseCaseOutput> {
        let settings = self.faq_repository.get_or_create_default_settings(tx)?;
        // validate
        let id = FaqItemId::from_str(&params.id).map_err(|_| Error::InvalidRequest)?;
        let slug = Slug::from_str(&params.slug)?;
        let item_with_contents_and_categories = self
            .faq_repository
            .get_item_with_contents_and_categories_by_id(tx, &id)?;
        if item_with_contents_and_categories.is_none() {
            return Err(Error::InvalidRequest);
        }
        let mut item_with_contents_and_categories = item_with_contents_and_categories.unwrap();
        if item_with_contents_and_categories.slug != slug
            && self.faq_repository.get_item_by_slug(tx, &slug)?.is_some()
        {
            return Err(Error::DuplicatedSlug);
        }
        let mut contents = vec![];
        for content_params in params.contents {
            let locale = FaqContentLocale::from_str(&content_params.locale)?;
            if !settings
                .data
                .supported_locales
                .iter()
                .any(|supported_locale| *supported_locale == locale)
            {
                return Err(Error::InvalidRequest);
            }
            let title = FaqItemTitle::from_str(&content_params.title)?;
            let body = FaqItemBody::from_str(&content_params.body)?;
            contents.push(FaqItemContent {
                faq_item_id: item_with_contents_and_categories.id.clone(),
                locale,
                title,
                body,
            });
        }
        let mut contents_should_be_removed_from_public_search = vec![];
        let is_going_to_be_private =
            item_with_contents_and_categories.is_published && !params.is_published;
        for existing_content in &item_with_contents_and_categories.contents {
            if is_going_to_be_private {
                contents_should_be_removed_from_public_search.push(existing_content.clone());
                continue;
            }
            if !contents
                .iter()
                .any(|new_content| new_content.locale == existing_content.locale)
            {
                contents_should_be_removed_from_public_search.push(existing_content.clone());
            }
        }

        let mut categories = vec![];
        for category_id in params.categories {
            let attached_category = item_with_contents_and_categories
                .categories
                .iter()
                .find(|c| c.faq_category_id.as_str() == category_id.as_str());
            if let Some(category) = attached_category {
                categories.push(category.clone());
                continue;
            }
            let category_id =
                FaqCategoryId::from_str(&category_id).map_err(|_| Error::InvalidRequest)?;
            let category_with_contents = self
                .faq_repository
                .get_category_with_contents_by_id(tx, &category_id)?;
            if category_with_contents.is_none() {
                return Err(Error::InvalidRequest)?;
            }
            let category_with_contents = category_with_contents.unwrap();
            let display_order = self
                .faq_repository
                .next_category_item_display_order(tx, &category_id)?;
            let category_item = FaqCategoryItem {
                faq_category_id: category_id,
                faq_item_id: item_with_contents_and_categories.id.clone(),
                display_order,
            };
            self.faq_repository
                .create_category_item(tx, &category_item)?;
            categories.push(FaqCategoryItemWithCategory::from((
                category_item,
                category_with_contents,
            )));
        }
        for existing_category_item in &item_with_contents_and_categories.categories {
            if categories.iter().any(|new_category_item| {
                new_category_item.faq_category_id.as_str()
                    == existing_category_item.faq_category_id.as_str()
            }) {
                continue;
            }
            self.faq_repository.delete_category_item(
                tx,
                FaqCategoryItem {
                    faq_category_id: existing_category_item.faq_category_id.clone(),
                    faq_item_id: existing_category_item.faq_item_id.clone(),
                    display_order: existing_category_item.display_order,
                },
            )?;
        }
        // update
        let now = Utc::now().naive_utc();
        let published_at = if params.is_published {
            if item_with_contents_and_categories.is_published {
                item_with_contents_and_categories.published_at
            } else {
                Some(now)
            }
        } else {
            None
        };
        self.faq_repository
            .update_item_with_contents_and_categories(
                tx,
                &mut item_with_contents_and_categories,
                slug,
                params.is_published,
                published_at,
                Some(now),
                contents,
                categories,
            )?;
        self.faq_search_repository
            .upsert_faq_item(&item_with_contents_and_categories)?;
        for faq_item_content in contents_should_be_removed_from_public_search {
            self.public_faq_search_repository
                .delete_faq_item_content(faq_item_content)?;
        }
        if item_with_contents_and_categories.is_published {
            for faq_item_content in &item_with_contents_and_categories.contents {
                self.public_faq_search_repository
                    .upsert_faq_item_content(faq_item_content)?;
            }
        }
        Ok(item_with_contents_and_categories)
    }
}
