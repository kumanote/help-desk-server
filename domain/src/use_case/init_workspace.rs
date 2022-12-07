use crate::{model::Locale, Result};

pub struct Input {
    pub email: String,
    pub password: String,
    pub locale: Locale,
}

/// initialize workspace.
/// * create workspace.
/// * initialize predefined roles.
/// * create first agent.
/// * create admin group and let first agent be an owner of the group.
pub fn execute(params: Input) -> Result<()> {
    todo!()
}
