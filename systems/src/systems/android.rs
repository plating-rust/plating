/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(any(feature = "android", doc))]

use plating_core::utils::{Deserialize, Serialize};

use crate::systems::SystemDefinition;
use crate::types::{ButtonAvailable, WindowAvailable};

/// [System Definition](`SystemDefinition`) for the Android OS.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported on Android.
///
/// See [`Android`] for the [System](`crate::systems::System`).
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "android"))]
pub struct AndroidDefinition {}
impl SystemDefinition for AndroidDefinition {}

impl ButtonAvailable for AndroidDefinition {}
impl WindowAvailable for AndroidDefinition {}

/// The Android [System](`crate::systems::System`)  .
///
/// Implements all the 'HasFoo' traits for types supported on Android.
///
/// # Availability
/// Only available on Android when 'android' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on Android.
///
/// # See also
/// See [`AndroidDefinition`] for the corresponding [`SystemDefinition`].
#[doc(cfg(feature = "android"))]
#[doc(cfg(target_os = "android"))]
#[cfg(any(target_os = "android", doc))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
pub struct Android {}

#[cfg(any(target_os = "android", doc))]
impl crate::systems::System for Android {
    type Definition = AndroidDefinition;
}
