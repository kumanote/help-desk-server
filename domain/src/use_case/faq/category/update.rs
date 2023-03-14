use crate::{
    model::{
        FaqCategoryContent, FaqCategoryId, FaqCategoryTitle, FaqCategoryWithContents,
        FaqContentLocale, Slug,
    },
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct UpdateFaqCategoryContent {
    pub locale: String,
    pub title: String,
}

pub struct UpdateFaqCategoryUseCaseInput {
    pub id: String,
    pub slug: String,
    pub contents: Vec<UpdateFaqCategoryContent>,
}

pub type UpdateFaqCategoryUseCaseOutput = FaqCategoryWithContents;

pub trait UpdateFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqCategoryUseCaseInput,
    ) -> Result<UpdateFaqCategoryUseCaseOutput>;
}

pub struct UpdateFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> UpdateFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> UpdateFaqCategoryUseCase
    for UpdateFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqCategoryUseCaseInput,
    ) -> Result<UpdateFaqCategoryUseCaseOutput> {
        let settings = self.faq_repository.get_or_create_default_settings(tx)?;
        // validate
        let id = FaqCategoryId::from_str(&params.id).map_err(|_| Error::InvalidRequest)?;
        let slug = Slug::from_str(&params.slug)?;
        let category_with_content = self
            .faq_repository
            .get_category_with_contents_by_id(tx, &id)?;
        if category_with_content.is_none() {
            return Err(Error::InvalidRequest);
        }
        let mut category_with_content = category_with_content.unwrap();
        if category_with_content.slug != slug
            && self
                .faq_repository
                .get_category_by_slug(tx, &slug)?
                .is_some()
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
            let title = FaqCategoryTitle::from_str(&content_params.title)?;
            contents.push(FaqCategoryContent {
                faq_category_id: category_with_content.id.clone(),
                locale,
                title,
            });
        }
        // update
        self.faq_repository.update_category_with_contents(
            tx,
            &mut category_with_content,
            slug,
            contents,
        )?;
        Ok(category_with_content)
    }
}
