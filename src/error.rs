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

use crate::widgets::System;
use std::error::Error;
use std::fmt;

/// Enum containing all kinds of errors that can occur dealing with plating.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[non_exhaustive]
pub enum PlatingErrorKind<S: System> {
    /// Error occurred in the backend.
    ///
    /// Contains a NativeError with more detailed information.
    BackendError(S::ErrorType),
}

/// The Error struct for plating
///
/// Contains a [`PlatingErrorKind`] with more detailed information.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PlatingError<S: System> {
    /// Holds more detailed Information.
    pub(crate) kind: PlatingErrorKind<S>,
}

impl<S: System> Error for PlatingError<S> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            PlatingErrorKind::BackendError(backend_error) => backend_error.source(),
        }
    }
}

impl<S: System> PlatingError<S> {
    /// Returns internal error kind.
    ///
    /// Useful to match against for more fine grained handling of errors
    pub fn kind(&self) -> &PlatingErrorKind<S> {
        &self.kind
    }
}

impl<S: System> fmt::Display for PlatingError<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PlatingErrorKind::BackendError(backend_error) => {
                write!(f, "BackendError: {}", backend_error)
            }
        }
    }
}
