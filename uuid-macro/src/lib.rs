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
pub use plating_uuid_macro_hack::uuid;

/// Noop macro used internally by plating.
///
/// ```rust
/// # use plating_macros::{noop};
/// //not much to tell, the following line does nothing
/// noop!();
/// ```
///
/// # Why
/// Used to typedef to macros when specific feature flags are disabled.
/// ```rust
/// #[cfg(feature = "log")]
/// pub(crate) use log::{debug, error, info, trace, warn};
/// #[cfg(not(feature = "log"))]
/// pub(crate) use plating_macros::{
///    noop as debug, noop as info, noop as warn, noop as error, noop as trace,
/// };
/// ```
#[proc_macro_hack]
pub use plating_uuid_macro_hack::noop;

pub use plating_uuid_macro_hack::bitflag_parameter;

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
