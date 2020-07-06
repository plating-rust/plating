/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

mod outlet_holder;
pub use outlet_holder::OutletHolder;

mod widget_pointer;
pub use widget_pointer::WidgetPointer;

mod child;
pub use child::Child;

/// Very basic trait implemented both by widgets themselves and
/// any kind of `Pointer` or other Widget indirection.
///
/// # Requirements
/// When implementing this trait, make sure that `name()`always returns the same value
/// and does not change during the lifetime of this instance.
pub trait Named {
    /// Get the name of this widget or the widget this object is pointing to.
    fn name(&self) -> &str;
}

/// Prelude for the widget::utils subsystem
///
/// Automatically included in ```plating::prelude::*``` and ```plating::widgets::prelude::*`
pub mod prelude {
    pub use super::Named;
}
