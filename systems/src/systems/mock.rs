#![cfg(any(feature = "mock", doc))]

use plating_core::mock::{MockButtonWidget, MockWindowWidget};
use plating_core::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use plating_core::utils::{Deserialize, Serialize};

use crate::systems::{System, SystemDefinition};
use crate::types::{ButtonAvailable, HasButton, HasWindow, WindowAvailable};

/// [System Definition](`SystemDefinition`) for the Mock System .
///
/// Contains the `FooAvailable` Trait implementations for all Widgets supported
/// by the [`Mock`] System.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "mock"))]
pub struct MockDefinition {}
impl SystemDefinition for MockDefinition {}

//TODO: autogenerate from HasButton
impl ButtonAvailable for MockDefinition {}

//TODO: autogenerate from HasTabs
impl WindowAvailable for MockDefinition {}

/// The Mock [System](`crate::systems::System`).
///
/// # Availability
/// Available on all platforms but only when 'ios' feature is enabled.
///
/// Is the [Native](`crate::Native`) type during tests.
///
/// TODO: A lot
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "mock"))]
pub struct Mock {}
impl System for Mock {
    type Definition = MockDefinition;
}
impl HasButton for Mock {
    type Button<OUTLET: OutletHolder> = MockButtonWidget<OUTLET>;
}
impl HasWindow for Mock {
    type Window<OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>> =
        MockWindowWidget<OUTLET>;
}
