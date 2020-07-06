/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WinErrorKind {
    TODO,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Error for WinError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        //todo: map to actual backend issue?!?!
        None
    }
}

impl fmt::Display for WinError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub type WinResult<T> = std::result::Result<T, WinError>;
