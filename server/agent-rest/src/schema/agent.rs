use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Agent {
    pub id: String,
    pub email: String,
    pub name: String,
    pub locale: String,
    pub is_active: bool,
}

impl From<model::Agent> for Agent {
    fn from(value: model::Agent) -> Self {
        Self {
            id: value.id.into(),
            email: value.email.into(),
            name: value.name.into(),
            locale: value.locale.into(),
            is_active: value.is_active,
        }
    }
}
