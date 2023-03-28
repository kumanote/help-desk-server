mod inquiry_thread_details;
mod inquiry_thread_status;
mod inquiry_thread_subject;

pub use inquiry_thread_details::*;
pub use inquiry_thread_status::*;
pub use inquiry_thread_subject::*;

use crate::model::{AgentId, InquiryChannelId, InquiryThreadId};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct InquiryThread {
    pub id: InquiryThreadId,
    pub inquiry_channel_id: InquiryChannelId,
    pub subject: InquiryThreadSubject,
    pub details: InquiryThreadDetails,
    pub status: InquiryThreadStatus,
    pub assigned_agent_id: Option<AgentId>,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}

impl<'a> Into<database::entities::NewInquiryThread<'a>> for &'a InquiryThread {
    fn into(self) -> database::entities::NewInquiryThread<'a> {
        database::entities::NewInquiryThread {
            id: &self.id,
            inquiry_channel_id: &self.inquiry_channel_id,
            subject: &self.subject,
            inquiry_thread_type: self.details.as_type(),
            details: (&self.details).into(),
            status: &self.status,
            assigned_agent_id: self.assigned_agent_id.as_deref(),
            opened_at: self.opened_at,
            closed_at: self.closed_at,
        }
    }
}

impl From<database::entities::InquiryThread> for InquiryThread {
    fn from(value: database::entities::InquiryThread) -> Self {
        Self {
            id: value.id.into(),
            inquiry_channel_id: value.inquiry_channel_id.into(),
            subject: value.subject.into(),
            details: value.details.into(),
            status: value.status.into(),
            assigned_agent_id: value.assigned_agent_id.map(Into::into),
            opened_at: value.opened_at,
            closed_at: value.closed_at,
        }
    }
}
