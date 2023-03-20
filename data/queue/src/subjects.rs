use std::fmt;

const EMAILS: &'static str = "emails";
const SEARCH: &'static str = "search";

#[derive(Debug, Clone)]
pub enum NatsSubject {
    /// Background email task.
    Emails,
    /// Background search engine update task.
    Search,
}

impl fmt::Display for NatsSubject {
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
