use std::fmt;

pub const SUBJECT_PREFIX_SEARCH: &'static str = "SEARCH";
pub const SUBJECT_PREFIX_INQUIRY_INCOMING_EVENTS: &'static str = "INQUIRY_INCOMING_EVENTS";

#[derive(Debug, Clone)]
pub enum NatsSubject {
    /// Background search engine update task.
    Search(NatsSearchSubject),
    /// Background handle inquiry webhook event task.
    InquiryIncomingEvents(NatsInquiryIncomingEventsSubject),
}

#[derive(Debug, Clone)]
pub enum NatsSearchSubject {
    FaqItem,
    PublicFaqItem,
}

#[derive(Debug, Clone)]
pub enum NatsInquiryIncomingEventsSubject {
    Line,
}

impl fmt::Display for NatsSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Search(inner_subject) => match inner_subject {
                NatsSearchSubject::FaqItem => write!(f, "{}.faq_item", SUBJECT_PREFIX_SEARCH),
                NatsSearchSubject::PublicFaqItem => {
                    write!(f, "{}.public_faq_item", SUBJECT_PREFIX_SEARCH)
                },
            },
            Self::InquiryIncomingEvents(inner_subject) => match inner_subject {
                NatsInquiryIncomingEventsSubject::Line => {
                    write!(f, "{}.line", SUBJECT_PREFIX_INQUIRY_INCOMING_EVENTS)
                },
            },
        }
    }
}
