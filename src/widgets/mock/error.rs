/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains [`MockError`] which is the error type for [`MockSystem`].

use std::error::Error;
use std::fmt;

/// A Mock error kind containing 3 different Error types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MockErrorKind {
    MockError,
}

/// Mock Error using [`MockErrorKind`] as the kind data.
///
/// Implements everything required by `System::ErrorType` to be used as a Systems error type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MockError {
    kind: MockErrorKind,
}
impl MockError {
    /// Returns internal error kind.
    ///
    /// Useful for to match against for more fine grained handling of errors
    pub fn kind(&self) -> &MockErrorKind {
        &self.kind
    }
}

/// `source()` always return `None` for MockError.
impl Error for MockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //todo: map to actual backend issue?!?!
        None
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Convenient typedef for the Result type used throughout the `mock` module
pub type MockResult<T> = std::result::Result<T, MockError>;
