/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Plating
//!
//! Big todo:
//! - introduce the different sub-projects
//! - Link to book and other resources
//! - Document feature flags
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    unused_import_braces,
    unused_crate_dependencies
)]
#![allow(incomplete_features)]
#![feature(doc_cfg)]

/// Constant representing the version of plating.
///
/// Useful for logging and debugging purposes.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg(any(feature = "cocoa", doc))]
#[doc(cfg(feature = "cocoa"))]
#[doc(inline)]
#[doc(alias = "apple")]
#[doc(alias = "mac")]
#[doc(alias = "osx")]
pub use cocoa;
pub use plating_core::*;
#[cfg(any(feature = "plating_systems", doc))]
#[doc(cfg(feature = "plating_systems"))]
#[doc(inline)]
pub use plating_systems::*;
#[cfg(any(feature = "win_ui3", doc))]
#[doc(cfg(feature = "win_ui3"))]
#[doc(inline)]
#[doc(alias = "windows")]
pub use win_ui3;
