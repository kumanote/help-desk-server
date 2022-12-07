use serde::{Deserialize, Serialize};

/// Parameters for background email task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Emails {
    ResetPassword {
        from: String,
        to: String,
        locale: String,
        subject: String,
        email: String,
        reset_password_link_url: String,
        valid_hours: i64,
    },
}
