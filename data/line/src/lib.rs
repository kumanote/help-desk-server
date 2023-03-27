mod error;
pub use error::*;
pub type Result<T> = std::result::Result<T, Error>;

mod client;
pub use client::*;

pub mod events;
pub mod messages;
pub mod objects;
