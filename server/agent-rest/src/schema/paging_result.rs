use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PagingResult<T: Sized + Serialize> {
    pub total: u64,
    pub list: Vec<T>,
}

impl<M, S> From<model::PagingResult<M>> for PagingResult<S>
where
    S: From<M> + Sized + Serialize,
{
    fn from(value: model::PagingResult<M>) -> Self {
        Self {
            total: value.total,
            list: value.list.into_iter().map(Into::into).collect(),
        }
    }
}
