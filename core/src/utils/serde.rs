/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Exposes Serialize and Deserialize derive macros.
//!
//! When the `serde` feature is enabled, export the Serialize and Deserialize
//! macros from the [`serde` crate](https://crates.io/crates/serde).
//!
//! When the `serde` feature is disabled, uses [noop_proc_macro](https://crates.io/crates/noop_proc_macro) Version of `Serialize`/`Deserialize`.
//!
//! ## Example
//! With both version the following will compile and either behave as normal
//! serde or do nothing:
//! ```rust,ignore
//! use plating::features::serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Example {
//!     foo: i32,
//!     bar: f64,
//! }
//!
//! fn main() {
//!     let example = Example { foo: 1, bar: 0.2 };
//!
//!     //we still have to use feature gate when serializing/deserializing
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&example).unwrap();
//!         let deserialized: Example = serde_json::from_str(&serialized).unwrap();
//!     }
//! }
//! ```
//!
//! See [`serde` documentation](https://docs.serde.rs/serde/) for further details.

#[cfg(feature = "serde")]
pub use serde::{
    de::Deserialize as DeserializeEx,
    de::DeserializeOwned as DeserializeTrait,
    ser::Serialize as SerializeTrait,
};
#[cfg(feature = "serde")]
pub use serde::{Deserialize, Serialize};

#[cfg(not(feature = "serde"))]
pub trait DeserializeTrait {}
#[cfg(not(feature = "serde"))]
pub use DeserializeTrait as DeserializeEx;

#[cfg(not(feature = "serde"))]
pub trait SerializeTrait {}

#[cfg(not(feature = "serde"))]
pub use noop_proc_macro::{Deserialize, Serialize};
