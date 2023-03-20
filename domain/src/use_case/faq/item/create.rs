use crate::{
    model::{
        FaqCategoryId, FaqCategoryItem, FaqCategoryItemWithCategory, FaqContentLocale, FaqItem,
        FaqItemBody, FaqItemContent, FaqItemId, FaqItemTitle, FaqItemWithContentsAndCategories,
        Slug,
    },
    repository::{FaqRepository, FaqSearchRepository, PublicFaqSearchRepository},
    Error, Result,
};
use chrono::Utc;
use std::str::FromStr;

pub struct CreateFaqItemContent {
    pub locale: String,
    pub title: String,
    pub body: String,
}

pub struct CreateFaqItemUseCaseInput {
    pub slug: String,
    pub is_published: bool,
    pub contents: Vec<CreateFaqItemContent>,
    /// category ids
    pub categories: Vec<String>,
}

pub type CreateFaqItemUseCaseOutput = FaqItemWithContentsAndCategories;

pub trait CreateFaqItemUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    type PublicFaqSearchRepository: PublicFaqSearchRepository<Err = Error>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateFaqItemUseCaseInput,
    ) -> Result<CreateFaqItemUseCaseOutput>;
}

pub struct CreateFaqItemUseCaseImpl<
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
    > CreateFaqItemUseCaseImpl<FR, FSR, PFSR>
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
    > CreateFaqItemUseCase for CreateFaqItemUseCaseImpl<FR, FSR, PFSR>
{
    type Transaction = TX;
    type FaqRepository = FR;
    type FaqSearchRepository = FSR;
    type PublicFaqSearchRepository = PFSR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateFaqItemUseCaseInput,
    ) -> Result<CreateFaqItemUseCaseOutput> {
        let settings = self.faq_repository.get_or_create_default_settings(tx)?;
        // validate
        let slug = Slug::from_str(&params.slug)?;
        if self.faq_repository.get_item_by_slug(tx, &slug)?.is_some() {
            return Err(Error::DuplicatedSlug);
        }
        let mut category_ids: Vec<FaqCategoryId> = vec![];
        for category_id in params.categories {
            let category_id =
                FaqCategoryId::from_str(&category_id).map_err(|_| Error::InvalidRequest)?;
            if self
                .faq_repository
                .get_category_by_id(tx, &category_id)?
                .is_none()
            {
                return Err(Error::InvalidRequest)?;
            }
            if category_ids.iter().any(|added| added == &category_id) {
                // duplicated check
                return Err(Error::InvalidRequest)?;
            }
            category_ids.push(category_id);
        }
        let mut content_details = vec![];
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
            content_details.push((locale, title, body));
        }
        // create faq item
        let now = Utc::now().naive_utc();
        let item = FaqItem {
            id: FaqItemId::generate(),
            slug,
            is_published: params.is_published,
            published_at: if params.is_published { Some(now) } else { None },
            last_updated_at: Some(now),
        };
        self.faq_repository.create_item(tx, &item)?;
        // create faq_item_contents
        let mut contents = vec![];
        for (locale, title, body) in content_details {
            let item_content = FaqItemContent {
                faq_item_id: item.id.clone(),
                locale,
                title,
                body,
            };
            self.faq_repository.create_item_content(tx, &item_content)?;
            contents.push(item_content);
        }
        // create faq_category_items
        let mut category_items = vec![];
        for category_id in category_ids {
            let display_order = self
                .faq_repository
                .next_category_item_display_order(tx, &category_id)?;
            let category_item = FaqCategoryItem {
                faq_category_id: category_id,
                faq_item_id: item.id.clone(),
                display_order,
            };
            self.faq_repository
                .create_category_item(tx, &category_item)?;
            let category_with_contents = self
                .faq_repository
                .get_category_with_contents_by_id(tx, &category_item.faq_category_id)?
                .unwrap();
            category_items.push(FaqCategoryItemWithCategory::from((
                category_item,
                category_with_contents,
            )));
        }

        let faq_item = FaqItemWithContentsAndCategories::from((item, contents, category_items));

        // update document for search engine.
        self.faq_search_repository.upsert_faq_item(&faq_item)?;
        if faq_item.is_published {
            for faq_item_content in &faq_item.contents {
                self.public_faq_search_repository
                    .upsert_faq_item_content(faq_item_content)?;
            }
        }

        Ok(faq_item)
    }
}
