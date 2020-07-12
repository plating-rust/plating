/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

//! internal crate for the macros used by plating.

use proc_macro_hack::proc_macro_hack;

/// Generate a compile time uuid.
///
/// ```rust
/// # use plating_macros::{uuid};
/// const GENERATED_UUID: &'static str = uuid!();
///
/// fn main() {
///     println!("{}", GENERATED_UUID);
/// }
/// ```
///
/// Uses [uuid](https://crates.io/crates/uuid) crate
#[proc_macro_hack]
pub use plating_uuid_macro_hack::{noop, uuid};

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_UUID: &'static str = uuid!();

    #[test]
    fn test_uuid_macro_valid() {
        let my_uuid: uuid::Uuid =
            uuid::Uuid::parse_str(GENERATED_UUID).expect("Uuid should have been valid.");

        assert_eq!(my_uuid.get_version_num(), 4);
    }

    #[test]
    fn test_uuid_macro_unique() {
        assert_ne!(uuid!(), uuid!());
    }

    #[test]
    fn noop() {
        noop!(); //not sure what to test, except it compiles
    }
}
