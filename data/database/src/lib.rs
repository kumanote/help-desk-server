#[macro_use]
extern crate diesel;

mod conn;
mod error;
mod pool;
mod schema;

pub use conn::*;
pub use error::*;
pub use pool::*;

pub mod adapters;
pub mod entities;

pub use diesel::connection::*;
pub type R2D2Error = r2d2::Error;
pub type DieselError = diesel::result::Error;
pub type DbConnection = diesel::mysql::MysqlConnection;
pub type DbConnectionPool = r2d2::Pool<diesel::r2d2::ConnectionManager<DbConnection>>;
pub type Result<T> = core::result::Result<T, Error>;
