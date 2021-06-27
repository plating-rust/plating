/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(any(feature = "qt", doc))]

use plating_core::utils::{Deserialize, Serialize};

use crate::systems::SystemDefinition;
use crate::types::{ButtonAvailable, WindowAvailable};

/// [System Definition](`SystemDefinition`) for the QT UI System.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported by QT.
///
/// See: [`QT`] for the [System](`crate::systems::System`).
/// For GTK see [`super::GTKDefinition`].
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "qt"))]
pub struct QTDefinition {}
impl SystemDefinition for QTDefinition {}

impl ButtonAvailable for QTDefinition {}
impl WindowAvailable for QTDefinition {}


/// The QT [System](`crate::systems::System`).
///
/// Implements all the 'HasFoo' traits for types supported by QT.
///
/// # Availability
/// Only available on Linux when 'qt' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on Android. (Only when 'gtk' feature
/// is not enabled at the same type).
///
/// # See also
/// See [`QTDefinition`] for the corresponding [`SystemDefinition`].

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "qt"))]
#[doc(cfg(target_os = "linux"))]
#[cfg(any(target_os = "linux", doc))]
#[doc(alias = "Linux")]
pub struct QT {}

#[cfg(any(target_os = "linux", doc))]
impl crate::systems::System for QT {
    type Definition = QTDefinition;
}
