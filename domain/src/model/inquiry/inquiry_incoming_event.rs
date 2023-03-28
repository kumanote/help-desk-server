#[derive(Debug, Clone)]
pub enum InquiryIncomingEvents {
    Line(Vec<line::events::Event>),
}

#[derive(Debug, Clone)]
pub enum InquiryIncomingEvent {
    Line(line::events::Event),
}

impl Into<queue::entities::InquiryIncomingEvent> for InquiryIncomingEvent {
    fn into(self) -> queue::entities::InquiryIncomingEvent {
        match self {
            Self::Line(event) => queue::entities::InquiryIncomingEvent::Line(event),
        }
    }
}
