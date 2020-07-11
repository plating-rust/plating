/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::fmt;
use thiserror::Error;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("CocoaError: {kind}")]
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
