/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing everything related to error handling in plating.
//! 
//! 
//! # Result Definitions
//! plating has type definitions for Result types for your convenience:
//! - [`NativeResult`](crate::NativeResult)
//! - [`PlatingResult`](crate::PlatingResult)
//! 
//! # Platforms
//! For Errors coming from the native parts, have look at those specific modules:
//! - [`cocoa::error`](crate::widgets::cocoa::error)
//! - [`windows::error`](crate::widgets::win::error)
//! - [`mock::error`](crate::widgets::mock::error)

use std::error::Error;
use std::fmt;
use crate::widgets::native::{NativeError};

/// Enum containing all kinds of errors that can occur dealing with plating.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlatingErrorKind {

    /// Error occurred win the backend. 
    /// 
    /// Contains a NativeError with more detailed information.
    BackendError(NativeError),
}

/// The Error struct for plating
/// 
/// Contains a [`PlatingErrorKind`] with more detailed information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlatingError {
    /// Holds more detailed Information.
    kind: PlatingErrorKind,
}

impl From<NativeError> for PlatingError {
    fn from(native_error: NativeError) -> Self {
        PlatingError{
            kind: PlatingErrorKind::BackendError(native_error)
        }
    }
}


impl Error for PlatingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            PlatingErrorKind::BackendError(backend_error) => backend_error.source()
        }
    }
}

impl PlatingError {
    /// Returns internal error kind.
    /// 
    /// Useful to match against for more fine grained handling of errors 
    pub fn kind(&self) -> &PlatingErrorKind {
        &self.kind
    }
}

impl fmt::Display for PlatingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PlatingErrorKind::BackendError(backend_error) => {
                write!(f, "BackendError: {}", backend_error)
            }
        }
    }
}
