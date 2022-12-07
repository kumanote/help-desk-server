mod client;
mod error;

pub use client::*;
pub use error::*;

pub mod adapters;
pub mod entities;

pub type SearchClient = meilisearch_sdk::client::Client;
pub type Result<T> = core::result::Result<T, Error>;
