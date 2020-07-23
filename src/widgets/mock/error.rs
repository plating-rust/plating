/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains [`MockError`] which is the error type for [`MockSystem`].

use crate::features::serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// A Mock error kind containing 3 different Error types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
#[non_exhaustive]
pub enum MockErrorKind {
    ///
    MockError1,
    ///
    MockError2,
    ///
    MockError3,
}
impl fmt::Display for MockErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MockError1 => write!(f, "MockError1"),
            Self::MockError2 => write!(f, "MockError2"),
            Self::MockError3 => write!(f, "MockError3"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[error("MockError: {kind}")]
pub struct MockError {
    kind: MockErrorKind,
}
impl MockError {
    /// Returns internal error kind.
    ///
    /// Useful for to match against for more fine grained handling of errors
    pub const fn kind(&self) -> &MockErrorKind {
        &self.kind
    }
}
