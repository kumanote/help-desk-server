use crate::model::{AgentId, InquiryContactId};

const TYPE_CONTACT: &'static str = "contact";
const TYPE_AGENT: &'static str = "agent";

#[derive(Debug, Clone)]
pub enum InquiryMessageSpeaker {
    Contact(InquiryContactId),
    Agent(AgentId),
}

impl InquiryMessageSpeaker {
    pub fn as_type(&self) -> &'static str {
        match self {
            Self::Contact(_) => TYPE_CONTACT,
            Self::Agent(_) => TYPE_AGENT,
        }
    }

    pub fn inquiry_contact_id(&self) -> Option<&InquiryContactId> {
        match self {
            Self::Contact(id) => Some(id),
            Self::Agent(_) => None,
        }
    }

    pub fn agent_id(&self) -> Option<&AgentId> {
        match self {
            Self::Contact(_) => None,
            Self::Agent(id) => Some(id),
        }
    }

    pub fn new(
        type_string: String,
        inquiry_contact_id: Option<String>,
        agent_id: Option<String>,
    ) -> Self {
        match type_string.as_str() {
            TYPE_CONTACT => Self::Contact(inquiry_contact_id.unwrap().into()),
            TYPE_AGENT => Self::Agent(agent_id.unwrap().into()),
            _ => unreachable!(),
        }
    }
}
