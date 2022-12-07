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
