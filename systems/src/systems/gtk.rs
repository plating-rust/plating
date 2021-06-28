/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(any(feature = "gtk", doc))]

use plating_core::utils::{Deserialize, Serialize};

use crate::systems::SystemDefinition;
use crate::types::{ButtonAvailable, WindowAvailable};

/// [System Definition](`SystemDefinition`) for the GTK UI System.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported by GTK.
///
/// See: [`GTK`] for the corresponding [System](`crate::systems::System`).
/// For QT see [`super::QTDefinition`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "gtk"))]
pub struct GTKDefinition {}
impl SystemDefinition for GTKDefinition {}

impl ButtonAvailable for GTKDefinition {}
impl WindowAvailable for GTKDefinition {}

/// The GTK [System](`crate::systems::System`).
///
/// Implements all the 'HasFoo' traits for types supported on GTK.
///
/// # Availability
/// Only available on Linux when 'gtk' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on Android. (Only when 'qt' feature is
/// not enabled at the same type).
///
/// # See also
/// See [`GTKDefinition`] for the corresponding [`SystemDefinition`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "gtk"))]
#[doc(cfg(target_os = "linux"))]
#[cfg(any(target_os = "linux", doc))]
#[doc(alias = "Linux")]
pub struct GTK {}

#[cfg(any(target_os = "linux", doc))]
impl crate::systems::System for GTK {
    type Definition = GTKDefinition;
}
