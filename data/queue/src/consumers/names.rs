use std::fmt;

const EMAILS: &'static str = "emails";
const SEARCH: &'static str = "search";

#[derive(Debug, Clone)]
pub enum ConsumerName {
    /// Consumer name(durable name) for background email task.
    Emails,
    /// Consumer name(durable name) for background search engine task.
    Search,
}

impl fmt::Display for ConsumerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Emails => {
                write!(f, "{}", EMAILS)
            },
            Self::Search => {
                write!(f, "{}", SEARCH)
            },
        }
    }
}
