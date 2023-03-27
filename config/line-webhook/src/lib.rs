mod error;
pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

mod args;
mod config;
mod toml;

pub use args::*;
pub use config::*;
