/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */
use crate::features::serde::{Deserialize, Serialize};

use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WinErrorKind {
    TODO,
}
impl fmt::Display for WinErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WinErrorKind::TODO => write!(f, "todo"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[error("WinError: {kind}")]
pub struct WinError {
    kind: WinErrorKind,
}
impl WinError {
    /// Returns internal error kind.
    ///
    /// Useful for to match against for more fine grained handling of errors
    pub fn kind(&self) -> &WinErrorKind {
        &self.kind
    }
}
