/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

/// When a UI element is created it needs to be added to a parent widget (except for Root Widgets).
/// When that's done, that does not mean a widget is actually shown because the parent (or a parent's parent...)
/// might not be added to the UI tree. We would call such a widget `unconnected`.
///
/// This trait should be implemented by all but root widgets for dealing with notification and handling of
/// connected status.
///
/// See [`Child`] for the trait regarding adding a parent.
pub trait Connectable {
    /// Will be called when this Widget gets connected directly or indirectly to the Root Element
    ///
    /// #Responsibilities
    /// Make sure [`connected`] returns true from now on.
    ///
    /// Should call connecting on all child elements, if they are connecting. Otherwise the implementer
    /// is responsible for calling `connecting` whenever child elements are actually connected.
    ///
    /// # Preconditions
    /// Will only be called when this element has a parent set. (See [`added`])
    /// Should 'panic' otherwise
    ///
    /// NOTE: this function will always be called after [`adding_to_parent`]
    ///
    /// NOTE: this is public so you can implement it for your own widgets. You should
    /// only call this function in the parent when the parent gets connected or when this was added as a child and the parent is already connected.
    fn connecting(&mut self);
    /// Will be called when this Widget gets connected directly or indirectly to the Root Element
    ///
    /// NOTE: this will always be called before any `removing_from_parent` functions are called
    fn disconnecting(&mut self);

    /// Returns true when the widget is currently connected
    ///
    /// See [`connect`] and [`disconnect`]
    fn connected(&self) -> bool;
}
