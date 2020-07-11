/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Connectable, Named};
use crate::widgets::{System, Widget};

/// Trait to be implemented by [`Widgets`](crate::widgets::Widget) that can be a [`Child`](crate::widgets::utils::Child) of another Widget.
/// That means all Widgets should implement it, except for `Root` Types.
///
/// # More
/// Learn more about adding and connection lifecycles in the [`actions::lifecycle`](crate::actions::lifecycle) module
pub trait Child<ParentType, ChildType, S>: Connectable
where
    ChildType: Named,
    ParentType: Widget<S> + Outlet<ChildType, S>,
    S: System,
{
    /// Will be called when this Widget gets added to a parent.
    ///
    /// #Responsibilities
    /// Setting the backend widget up so it has the correct parent set.
    ///
    /// NOTE: This does not mean that this widget will be actually displayed it could be that any of the parent
    /// elements are currently not `connected` to the Root Ui Element.
    ///
    /// # Preconditions
    /// Should panic if a parent is currently set.
    ///
    /// NOTE: this is public so you can implement it for your own widgets. You should
    /// only call this function in the parent when this widget is also added as a child.
    fn adding_to_parent(&mut self, _parent: &ParentType::ParentData);

    /// Removes a parent previously set via [`adding_to_parent`].
    ///
    /// #Preconditions
    /// A parent needs to be set.
    /// Should panic if called without a parent set.
    ///
    /// NOTE: this is public so you can implement it for your own widgets. You should
    /// only call this function in the parent when this widget is also removed as a child.
    fn removing_from_parent(&mut self);

    /// Returns true when the widget is currently added as a child
    ///
    /// See [`adding_to_parent`] and [`removing_from_parent`]
    fn added(&self) -> bool;
}
