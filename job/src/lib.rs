mod error;
pub use error::*;
pub type Result<T> = core::result::Result<T, Error>;

mod config;

mod executor;
pub use executor::*;
