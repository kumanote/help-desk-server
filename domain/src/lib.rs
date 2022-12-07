pub mod logic;
pub mod model;
pub mod repository;
pub mod use_case;

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
