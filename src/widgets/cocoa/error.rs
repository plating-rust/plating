/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use super::CocoaSystem;
use crate::error::{PlatingError, PlatingErrorKind};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[non_exhaustive]
pub enum CocoaErrorKind {
    TODO,
}
impl fmt::Display for CocoaErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CocoaErrorKind::TODO => write!(f, "todo"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CocoaError {
    kind: CocoaErrorKind,
}
impl CocoaError {
    /// Returns internal error kind.
    ///
    /// Useful for to match against for more fine grained handling of errors
    pub fn kind(&self) -> &CocoaErrorKind {
        &self.kind
    }
}

impl Error for CocoaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //todo: map to actual backend issue?!?!
        None
    }
}

impl fmt::Display for CocoaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CocoaError: {}", self.kind)
    }
}

impl From<CocoaError> for PlatingError<CocoaSystem> {
    fn from(native_error: CocoaError) -> Self {
        PlatingError {
            kind: PlatingErrorKind::BackendError(native_error),
        }
    }
}

pub type CocoaResult<T> = std::result::Result<T, CocoaError>;
