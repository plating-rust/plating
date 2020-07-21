/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */
#![deny(missing_docs)]

//! General module for useful small structs, functions or traits.
//!
//! Contains mostly traits that are used generic Widget Traits as well as the platform specific widget implementations.
//!
//!
//! NOTE: Platform dependant utils should be in their respective submodule.

mod outlet_holder;
pub use outlet_holder::OutletHolder;

mod child;
pub use child::*;

mod connectable;
pub use connectable::*;

mod parameters;
pub use parameters::*;

/// Very basic trait implemented both by widgets themselves and
/// any kind of `Pointer` or other Widget indirection.
///
/// # Requirements
/// When implementing this trait, make sure that `id()`always returns the same value
/// and does not change during the lifetime of this instance.
///
/// NOTE: This Trait is a requirement for the [`Widget`](crate::widgets::Widget) Trait.
pub trait Identity {
    /// Get the id of this widget or the widget this object is pointing to.
    fn id(&self) -> &str;
}

/// Prelude for the widget::utils subsystem
///
/// Automatically included in the following preludes:
/// - [`plating::prelude::*`](crate::prelude) and
/// - [`plating::widgets::prelude::*`](crate::widgets::prelude)
pub mod prelude {
    pub use super::Child;
    pub use super::Identity;
    pub use super::Parameters;
}
