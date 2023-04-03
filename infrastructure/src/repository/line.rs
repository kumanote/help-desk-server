use domain::model::InquiryLineProfile;
use domain::repository::LineRepository;
use futures::executor;
use line::LineClient;

pub struct LineRepositoryImpl {
    line_client: LineClient,
}

impl LineRepositoryImpl {
    pub fn new(line_client: LineClient) -> Self {
        Self { line_client }
    }
}

impl LineRepository for LineRepositoryImpl {
    type Err = domain::Error;

    fn get_profile(&self, line_user_id: &str) -> Result<Option<InquiryLineProfile>, Self::Err> {
        match executor::block_on(self.line_client.get_profile(line_user_id)) {
            Ok(profile) => Ok(Some(profile)),
            Err(err) => match &err {
                line::Error::ErrorResponse(error_response) => {
                    if error_response.message == "Not found" {
                        Ok(None)
                    } else {
                        Err(err.into())
                    }
                },
                _ => Err(err.into()),
            },
        }
    }
}
