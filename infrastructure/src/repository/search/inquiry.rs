use anyhow::anyhow;
use domain::model::{InquiryContact, InquiryMessage};
use domain::repository::InquirySearchRepository;
use futures::executor;
use search::SearchClient;

pub struct InquirySearchRepositoryImpl {
    search_client: SearchClient,
}

impl InquirySearchRepositoryImpl {
    pub fn new(search_client: SearchClient) -> Self {
        Self { search_client }
    }
}

impl InquirySearchRepository for InquirySearchRepositoryImpl {
    type Err = domain::Error;

    fn upsert_inquiry_contact(&self, contact: &InquiryContact) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::inquiry_contact::add_or_replace(
            &self.search_client,
            contact.into(),
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete upsert inquiry_contact meilisearch document."),
            }),
        }
    }

    fn delete_inquiry_contact(&self, contact: &InquiryContact) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::inquiry_contact::delete_by_id(
            &self.search_client,
            &contact.id,
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete delete inquiry_contact meilisearch document."),
            }),
        }
    }

    fn upsert_inquiry_message(&self, message: &InquiryMessage) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::inquiry_message::add_or_replace(
            &self.search_client,
            message.into(),
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete upsert inquiry_message meilisearch document."),
            }),
        }
    }

    fn delete_inquiry_message(&self, message: &InquiryMessage) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::inquiry_message::delete_by_id(
            &self.search_client,
            &message.id,
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete delete inquiry_message meilisearch document."),
            }),
        }
    }
}
