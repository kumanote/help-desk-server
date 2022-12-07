use crate::{DbConnection, Result};
use diesel::prelude::*;

pub fn establish_connection<S: Into<String>>(database_url: S) -> Result<DbConnection> {
    let database_url = database_url.into();
    Ok(DbConnection::establish(&database_url)?)
}
