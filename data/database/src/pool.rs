use crate::{DbConnection, DbConnectionPool, Result};
use diesel::r2d2::{ConnectionManager, Pool};

pub fn new_pool<S: Into<String>>(database_url: S, max_size: u32) -> Result<DbConnectionPool> {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    Pool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(Into::into)
}
