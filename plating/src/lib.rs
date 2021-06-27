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
    //missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    unused_import_braces,
    unused_crate_dependencies
)]

/// Constant representing the version of plating.
///
/// Useful for logging and debugging purposes.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


pub use plating_core::*;
#[cfg(feature = "plating_systems")]
pub use plating_systems::*;
