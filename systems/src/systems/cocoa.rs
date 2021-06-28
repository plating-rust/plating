/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(any(feature = "cocoa", doc))]

//TODO; use real ones

#[cfg(target_os = "macos")]
use plating_core::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};

use plating_core::utils::{Deserialize, Serialize};
use crate::systems::SystemDefinition;
use crate::types::{ButtonAvailable, WindowAvailable};
#[cfg(target_os = "macos")]
use crate::types::{HasButton, HasWindow};

/// [System Definition](`SystemDefinition`) for the Cocoa UI System.
///
/// [`Cocoa`] is the UI System for MacOS.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported on Mac.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "cocoa"))]
pub struct CocoaDefinition {}

impl SystemDefinition for CocoaDefinition {}
impl ButtonAvailable for CocoaDefinition {}
impl WindowAvailable for CocoaDefinition {}


/// The Cocoa [System](`crate::systems::System`).
///
/// Implements all the 'HasFoo' traits for types supported on Mac.
///
/// # Availability
/// Only available on MacOs when 'cocoa' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on MacOs.
///
/// # See also
/// See [`CocoaDefinition`] for the corresponding [`SystemDefinition`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "cocoa"))]
#[doc(cfg(target_os = "macos"))]
#[cfg(target_os = "macos")]
#[doc(alias = "Mac")]
#[doc(alias = "Osx")]
pub struct Cocoa {}

#[cfg(target_os = "macos")]
impl crate::systems::System for Cocoa {
    type Definition = CocoaDefinition;
}

#[cfg(target_os = "macos")]
impl HasButton for Cocoa {
    type Button<OUTLET: OutletHolder> = plating_core::mock::MockButtonWidget<OUTLET>;
}

#[cfg(target_os = "macos")]
impl HasWindow for Cocoa {
    type Window<OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>> =
        plating_core::mock::MockWindowWidget<OUTLET>;
}
/*
pub trait CocoaSpecific {
    type Button<OUTLET: OutletHolder>: Button<OUTLET>;
    type Window<OUTLET: OutletHolder
                      + Outlet<MenuOutlet>
                      + Outlet<ChildrenOutlet>>: Window<OUTLET>;
}
impl CocoaSpecific for Cocoa {
    type Button<OUTLET: OutletHolder> = MockButtonWidget<OUTLET>;
    type Window<OUTLET: OutletHolder
                      + Outlet<MenuOutlet>
                      + Outlet<ChildrenOutlet>> = MockWindowWidget<OUTLET>;
}*/
