use crate::subjects::{NatsInquiryIncomingEventsSubject, NatsSubject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InquiryIncomingEvent {
    Line(line::events::Event),
}

impl InquiryIncomingEvent {
    pub fn get_subject(&self) -> NatsSubject {
        match self {
            Self::Line(_) => {
                NatsSubject::InquiryIncomingEvents(NatsInquiryIncomingEventsSubject::Line)
            },
        }
    }
}
