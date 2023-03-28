use std::fmt;

pub const SEARCH: &'static str = "SEARCH";
pub const INQUIRY_INCOMING_EVENTS: &'static str = "INQUIRY_INCOMING_EVENTS";

#[derive(Debug, Clone)]
pub enum NatsStream {
    Search,
    InquiryIncomingEvents,
}

impl fmt::Display for NatsStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Search => write!(f, "{}", SEARCH),
            Self::InquiryIncomingEvents => write!(f, "{}", INQUIRY_INCOMING_EVENTS),
        }
    }
}
