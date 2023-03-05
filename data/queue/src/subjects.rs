use std::fmt;

const EMAILS: &'static str = "emails";

#[derive(Debug, Clone)]
pub enum NatsSubject {
    /// Background email task.
    Emails,
}

impl fmt::Display for NatsSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Emails => {
                write!(f, "{}", EMAILS)
            },
        }
    }
}
