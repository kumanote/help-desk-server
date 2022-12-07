use std::fmt;

const EMAILS: &'static str = "emails";

#[derive(Debug, Clone)]
pub enum ConsumerName {
    /// Consumer name(durable name) for background email task.
    Emails,
}

impl fmt::Display for ConsumerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Emails => {
                write!(f, "{}", EMAILS)
            }
        }
    }
}
