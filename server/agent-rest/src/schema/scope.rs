use domain::model;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Scope(String);

impl From<model::Scope> for Scope {
    fn from(value: model::Scope) -> Self {
        Self(value.into())
    }
}
