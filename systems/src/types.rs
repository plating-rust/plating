use plating_core::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use plating_core::widgets::{Button, Window};

use crate::marker;
use crate::systems::{System, SystemDefinition, SystemsList};
use crate::tags::Tag;

marker! {
    /// The window type for Systems supporting them.
    pub type Window<OUTLET: OutletHolder
                          + Outlet<MenuOutlet>
                          + Outlet<ChildrenOutlet>>: Window<OUTLET>;
}
marker! {
    /// The button type for Systems supporting them.
    pub type Button<OUTLET: OutletHolder>: Button<OUTLET>;
}
