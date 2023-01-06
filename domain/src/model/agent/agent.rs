use crate::model::{AgentId, AgentName, Email, HashedPassword, Locale};

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub name: AgentName,
    pub locale: Locale,
    pub is_active: bool,
}

impl<'a> Into<database::entities::NewAgent<'a>> for &'a Agent {
    fn into(self) -> database::entities::NewAgent<'a> {
        database::entities::NewAgent {
            id: &self.id,
            email: &self.email,
            hashed_password: &self.hashed_password,
            name: &self.name,
            locale: &self.locale,
            is_active: self.is_active,
        }
    }
}

impl From<database::entities::Agent> for Agent {
    fn from(value: database::entities::Agent) -> Self {
        Self {
            id: value.id.into(),
            email: value.email.into(),
            hashed_password: value.hashed_password.into(),
            name: value.name.into(),
            locale: value.locale.into(),
            is_active: value.is_active,
        }
    }
}
