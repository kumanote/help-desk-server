use crate::model::InquiryLineProfile;

pub trait LineRepository: Send + Sync + 'static {
    type Err;
    fn get_profile(&self, line_user_id: &str) -> Result<Option<InquiryLineProfile>, Self::Err>;
}
