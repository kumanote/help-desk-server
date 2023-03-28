use crate::stream::NatsStream;
use std::fmt;

const ALL_SUBJECT: &'static str = "";

const ALL_SEARCH: &'static str = "ALL_SEARCH";
const ALL_INQUIRY_INCOMING_EVENTS: &'static str = "ALL_INQUIRY_INCOMING_EVENTS";

#[derive(Debug, Clone)]
pub enum ConsumerName {
    /// Consumer name(durable name) for background search engine task.
    AllSearch,
    /// Consumer name(durable name) for background handle inquiry incoming event task.
    AllInquiryIncomingEvents,
}

impl ConsumerName {
    pub fn get_stream(&self) -> NatsStream {
        match self {
            Self::AllSearch => NatsStream::Search,
            Self::AllInquiryIncomingEvents => NatsStream::InquiryIncomingEvents,
        }
    }

    pub fn get_subject(&self) -> &str {
        match self {
            Self::AllSearch => ALL_SUBJECT,
            Self::AllInquiryIncomingEvents => ALL_SUBJECT,
        }
    }
}

impl fmt::Display for ConsumerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllSearch => {
                write!(f, "{}", ALL_SEARCH)
            },
            Self::AllInquiryIncomingEvents => {
                write!(f, "{}", ALL_INQUIRY_INCOMING_EVENTS)
            },
        }
    }
}
