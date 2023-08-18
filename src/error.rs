#![allow(deprecated)]

use trackable::error::{Failed, TrackableError};

/// This crate specific `Error` type.
#[derive(Debug, Clone)]
pub struct Error(TrackableError<Failed>);
derive_traits_for_trackable_error_newtype!(Error, Failed);
