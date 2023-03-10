use crate::{
    model::{
        FaqCategory, FaqCategoryContent, FaqCategoryId, FaqCategoryTitle, FaqCategoryWithContents,
        FaqContentLocale, Slug,
    },
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct CreateFaqCategoryContent {
    pub locale: String,
    pub title: String,
}

pub struct CreateFaqCategoryUseCaseInput {
    pub slug: String,
    pub contents: Vec<CreateFaqCategoryContent>,
}

pub type CreateFaqCategoryUseCaseOutput = FaqCategoryWithContents;

pub trait CreateFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateFaqCategoryUseCaseInput,
    ) -> Result<CreateFaqCategoryUseCaseOutput>;
}

pub struct CreateFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> CreateFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> CreateFaqCategoryUseCase
    for CreateFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateFaqCategoryUseCaseInput,
    ) -> Result<CreateFaqCategoryUseCaseOutput> {
        let settings = self.faq_repository.get_or_create_default_settings(tx)?;
        // validate
        let slug = Slug::from_str(&params.slug)?;
        if self
            .faq_repository
            .get_category_by_slug(tx, &slug)?
            .is_some()
        {
            return Err(Error::DuplicatedSlug);
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
            let title = FaqCategoryTitle::from_str(&content_params.title)?;
            content_details.push((locale, title));
        }
        // create faq category
        let category = FaqCategory {
            id: FaqCategoryId::generate(),
            slug,
            display_order: self.faq_repository.next_category_display_order(tx)?,
        };
        self.faq_repository.create_category(tx, &category)?;
        // create contents
        let mut contents = vec![];
        for (locale, title) in content_details {
            let category_content = FaqCategoryContent {
                faq_category_id: category.id.clone(),
                locale,
                title,
            };
            self.faq_repository
                .create_category_content(tx, &category_content)?;
            contents.push(category_content);
        }
        Ok(CreateFaqCategoryUseCaseOutput::from((category, contents)))
    }
}
