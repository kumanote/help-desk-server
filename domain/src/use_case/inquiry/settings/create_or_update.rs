use crate::{
    model::{
        InquiryLineSettings, InquiryNotificationSettings, InquirySettings, InquirySettingsData, Url,
    },
    repository::InquiryRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct CreateOrUpdateInquirySettingsUseCaseInput {
    pub line: InquiryLineSettingsInput,
    pub notification: InquiryNotificationSettingsInput,
}

pub struct InquiryLineSettingsInput {
    pub enabled: bool,
    pub friend_url: Option<String>,
    pub friend_qr_code_url: Option<String>,
}

impl TryInto<InquiryLineSettings> for InquiryLineSettingsInput {
    type Error = Error;
    fn try_into(self) -> Result<InquiryLineSettings> {
        let friend_url = if let Some(friend_url) = self.friend_url.as_deref() {
            Some(Url::from_str(friend_url)?)
        } else {
            None
        };
        let friend_qr_code_url =
            if let Some(friend_qr_code_url) = self.friend_qr_code_url.as_deref() {
                Some(Url::from_str(friend_qr_code_url)?)
            } else {
                None
            };
        Ok(InquiryLineSettings {
            enabled: self.enabled,
            friend_url,
            friend_qr_code_url,
        })
    }
}

pub struct InquiryNotificationSettingsInput {
    pub slack_webhook_url: Option<String>,
}

impl TryInto<InquiryNotificationSettings> for InquiryNotificationSettingsInput {
    type Error = Error;
    fn try_into(self) -> Result<InquiryNotificationSettings> {
        let slack_webhook_url = if let Some(slack_webhook_url) = self.slack_webhook_url.as_deref() {
            Some(Url::from_str(slack_webhook_url)?)
        } else {
            None
        };
        Ok(InquiryNotificationSettings { slack_webhook_url })
    }
}

pub type CreateOrUpdateInquirySettingsUseCaseOutput = InquirySettings;

pub trait CreateOrUpdateInquirySettingsUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateOrUpdateInquirySettingsUseCaseInput,
    ) -> Result<CreateOrUpdateInquirySettingsUseCaseOutput>;
}

pub struct CreateOrUpdateInquirySettingsUseCaseImpl<FR: InquiryRepository<Err = Error>> {
    inquiry_repository: FR,
}

impl<FR: InquiryRepository<Err = Error>> CreateOrUpdateInquirySettingsUseCaseImpl<FR> {
    pub fn new(inquiry_repository: FR) -> Self {
        Self { inquiry_repository }
    }
}

impl<TX, FR: InquiryRepository<Err = Error, Transaction = TX>> CreateOrUpdateInquirySettingsUseCase
    for CreateOrUpdateInquirySettingsUseCaseImpl<FR>
{
    type Transaction = TX;
    type InquiryRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: CreateOrUpdateInquirySettingsUseCaseInput,
    ) -> Result<CreateOrUpdateInquirySettingsUseCaseOutput> {
        let mut settings = match self.inquiry_repository.get_settings(tx)? {
            Some(settings) => settings,
            None => InquirySettings::default(),
        };
        // validate inputs
        let line: InquiryLineSettings = params.line.try_into()?;
        let notification: InquiryNotificationSettings = params.notification.try_into()?;
        let data = InquirySettingsData { line, notification };

        // update
        self.inquiry_repository
            .upsert_settings(tx, &mut settings, data)?;
        Ok(settings)
    }
}
