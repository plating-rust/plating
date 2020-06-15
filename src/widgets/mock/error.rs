/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MockErrorKind {
    MockError,
}

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


pub type MockResult<T> = std::result::Result<T, MockError>;
