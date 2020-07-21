/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! # Plating project
//!
//! ![Version: Alpha](https://img.shields.io/badge/version-alpha-red.svg)
//! [![Book](https://img.shields.io/badge/book-master-green.svg)](https://plating-rust.github.io/)
//! [![Docs](https://docs.rs/plating/badge.svg)](https://docs.rs/plating/)
//!
//! todo: top level documentation
//!
//! ## Versioning
//! I am using `0.0.x` for the alpha version. They will contain breaking changes, use at your own risk!<br>
//! The plan is to use [Semver](https://semver.org/) from `^0.1` onward
//! ## Supported Platforms:
//! Currently ***none*** as this is still ![Alpha](https://img.shields.io/badge/version-alpha-red.svg) (:<br>
//! I'm working on macos first with Windows and Linux to follow.
//!
//! The plan is as follows:
//! 3 Tier System:
//! - Tier 1: major Desktop OSs: *Macos*, *Window*, *Linux* (Gnome + Gtk)
//! - Tier 2: Android and IOs to some extend. The focus will be on desktop environments.
//! - Tier 3: community based.
//! ## Features
//! - **`serde`**: Enables `derive(Serialize, Deserialize)` on all the `Parameter` structs required to create the widgets.
//! - **`log`**: Some internal debug logging. For internal purposes mostly.
//! - **`mock_os`**: If enabled, uses Mock widgets and typedefs them in the [native](crate::widgets::native)
//!   module. Useful for integration tests<br>
//!   *NOTE*: The widgets for the current platform will still be available under `widgets::{os_specific_module}`.<br>
//!   *NOTE*: Even without this feature, the mock widgets are still available under the [mock module](crate::widgets::mock).
//!
//! Default features are `serde` and `log`.
//! ## Local Documentation
//! If you want to create this documentation locally, you need
//! the nightly toolchain. (or the internal links will not work).
//! Alternatively you can run create it via `rustup run nightly cargo rustdoc` and keep your current toolchain.
//!
//! #### License
//!
//! Licensed under either of *Apache License, Version 2.0* or *MIT license* at your option.
//! <br>
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in `plating` by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.
//!

#![warn(clippy::cargo_common_metadata)]
#![deny(
    clippy::cognitive_complexity,
    clippy::fallible_impl_from,
    clippy::missing_const_for_fn
)]
#![feature(doc_cfg)]
#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]
//#![deny(missing_doc_code_examples)]
#![doc(issue_tracker_base_url = "https://github.com/plating-rust/plating/issues/")]

/////////////////////////
// extern crates
/////////////////////////
#[macro_use]
extern crate bitflags;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(all(not(target_os = "macos")))]
compile_error!("Unsupported platform.");

pub mod actions;
pub mod events;
pub mod widgets;

mod data;
pub use data::*;

/// Includes traits implemented by Widgets. That way you can use most functionality
/// of widgets without explicitly importing them.
///
/// # Best practice
/// use the following line to make the traits available
/// ```rust
/// use plating::prelude::*;
/// ```
pub mod prelude {
    pub use crate::widgets::prelude::*;
}

/// Define feature specific objects with the goal to remove `#cfg[]` as much as possible from actual implementation code.
///
/// Used for the `log` and `serde` features.
pub(crate) mod features {
    /// Exposes logging macros that might evaluate to noop when feature `log` is not enabled.
    ///
    /// When `log` feature is enabled, use the
    /// `info`, `warn`, `trace`, `error` macros
    /// from the [log](https://crates.io/crates/log) crate. See the documentation of `log`
    /// on how to setup logging in your app.
    ///
    /// When `log` feature is disabled, use macros that evaluate to noop.
    /// (See [noop_attr](https://crates.io/crates/noop-attr) crate)
    ///
    /// ## Example
    /// ```rust,ignore
    /// use plating::features::log::{debug, info, warn, error, trace};
    ///
    /// fn main() {
    ///     debug!("Display an info: {}", 1);
    ///     info!("Display an info: {}", 2);
    ///     warn!("Display an info: {}", 3);
    ///     error!("Display an info: {}", 4);
    ///     trace!("Display an info: {}", 4);
    /// }
    /// ```
    ///
    /// for more detailed usage info, look at the [documentation of log](https://docs.rs/log/)
    pub(crate) mod log {
        #[allow(unused_imports)] //todo remove!
        #[cfg(feature = "log")]
        pub(crate) use log::{debug, error, info, trace, warn};
        #[cfg(not(feature = "log"))]
        pub(crate) use plating_macros::{
            noop as debug, noop as info, noop as warn, noop as error, noop as trace,
        };
    }

    /// Exposes Serialize and Deserialize derive macros.
    ///
    /// When the `serde` feature is enabled, export the Serialize and Deserialize macros
    /// from the [`serde` crate](https://crates.io/crates/serde).
    ///
    /// When the `serde` feature is disabled, uses [noop_proc_macro](https://crates.io/crates/noop_proc_macro) Version of `Serialize`/`Deserialize`.
    ///
    /// ## Example
    /// With both version the following will compile and either behave as normal serde
    /// or do nothing:
    /// ```rust,ignore
    /// use plating::features::serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Example {
    ///     foo: i32,
    ///     bar: f64,
    /// }
    ///
    /// fn main() {
    ///     let example = Example { foo: 1, bar: 0.2 };
    ///
    ///     //we still have to use feature gate when serializing/deserializing
    ///     #[cfg(feature = "serde")]
    ///     {
    ///         let serialized = serde_json::to_string(&example).unwrap();
    ///         let deserialized: Example = serde_json::from_str(&serialized).unwrap();
    ///     }
    /// }
    /// ```
    ///
    /// See [`serde` documentation](https://docs.serde.rs/serde/) for further details.
    pub(crate) mod serde {
        #[cfg(feature = "serde")]
        pub(crate) use serde::{Deserialize, Serialize};

        #[cfg(feature = "serde")]
        pub(crate) use serde::{
            de::Deserialize as DeserializeTrait, ser::Serialize as SerializeTrait,
        };

        #[cfg(not(feature = "serde"))]
        pub(crate) use noop_proc_macro::{Deserialize, Serialize};
    }
}

/// Convenience definition for ```Result<T, anyhow::Error>```
pub type PlatingResult<T> = std::result::Result<T, anyhow::Error>;

pub use plating_macros::uuid;
