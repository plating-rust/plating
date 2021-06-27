#![cfg(any(feature = "ios", doc))]

use plating_core::utils::{Deserialize, Serialize};

use crate::systems::SystemDefinition;
use crate::types::ButtonAvailable;

/// [System Definition](`SystemDefinition`) for IOS.
///
/// Contains the `FooAvailable` Trait implementations for all Widgets natively
/// supported on IOS.
///
/// See [`IOS`] for the [System](`crate::systems::System`).
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
#[doc(cfg(feature = "ios"))]
pub struct IOSDefinition {}
impl SystemDefinition for IOSDefinition {}

impl ButtonAvailable for IOSDefinition {}

/// The IOS [System](`crate::systems::System`).
///
/// Implements all the 'HasFoo' traits for types supported on IOS.
///
/// # Availability
/// Only available on IOS when 'ios' feature is enabled.
///
/// Is the [Native](`crate::Native`) type on IOS.
///
/// # See also
/// See [`IOSDefinition`] for the corresponding [`SystemDefinition`].
#[doc(cfg(feature = "ios"))]
#[doc(cfg(target_os = "ios"))]
#[cfg(any(target_os = "ios", doc))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
pub struct IOS {}

#[cfg(any(target_os = "ios", doc))]
impl crate::systems::System for IOS {
    type Definition = IOSDefinition;
}
