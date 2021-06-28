/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! crate level documentation still missing

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![warn(unused_crate_dependencies)]

// we don't want to keep unused dependencies around, but cargo_husky is needed
// for git hooks even though we don't really use it in our codebase.
// hence we `fake` use it
#[cfg(test)]
use cargo_husky as _;
