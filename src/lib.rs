#[macro_use]
extern crate trackable;

pub use error::Error;

mod error;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
