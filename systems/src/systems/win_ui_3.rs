/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(any(feature = "winui3", doc))]

use plating_core::utils::{Deserialize, Serialize};

use crate::systems::SystemDefinition;
use crate::types::{ButtonAvailable, WindowAvailable};

/// [System Definition](`SystemDefinition`) for the WinUI3 System.
///
/// [`WinUI3`] is the UI System for Windows.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported on WinUI3.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "winui3"))]
pub struct WinUI3Definition {}
impl SystemDefinition for WinUI3Definition {}

impl ButtonAvailable for WinUI3Definition {}
impl WindowAvailable for WinUI3Definition {}

/// The WinUI3 [System](`crate::systems::System`).
///
/// Implements all the 'HasFoo' traits for types supported by WinUI3 on Windows.
///
/// # Availability
/// Only available on Windows when 'winui3' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on Windows.
///
/// # See also
/// See [`WinUI3Definition`] for the corresponding [`SystemDefinition`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "winui3"))]
#[doc(cfg(target_os = "windows"))]
#[cfg(any(target_os = "windows", doc))]
#[doc(alias = "Windows")]
pub struct WinUI3 {}

#[cfg(target_os = "windows")]
impl crate::systems::System for WinUI3 {
    type Definition = CocoaDefinition;
}
