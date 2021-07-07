/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::marker::PhantomData;

use plating_core::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use plating_core::utils::{Deserialize, Serialize};
use plating_core::widgets::{Button, Window};

use crate::systems::{SystemDefinition, SystemsList};
use crate::types::{ButtonAvailable, HasButton, HasWindow, WindowAvailable};
use crate::{default_widgets, tag};

/// A marker Trait for Tags
///
/// If you want to implement your custom version of [`Tag`], you need to
/// implement this Trait as well. See [`Tag`] for information on its use and
/// requirements.
///
/// # How to Implement this Trait
/// ```
/// use plating_systems::tags::TagTrait;
///
/// struct Foo {}
///
/// impl TagTrait for Foo {}
/// ````
///
/// See also [`Tag<Sys>`]
pub trait TagTrait {}

/// A 'Tag<Sys>' represents a list of Systems.
///
/// ## Create Tag
/// See [`tag`] macro for the preferred way to create one. This will create the
/// Trait for that system as well.
///
/// Alternatively you can create one by hand:
/// ```
/// # #[cfg(all(feature = "cocoa", feature = "winui3"))]
/// use plating_systems::systems::{CocoaDefinition, WinUI3Definition};
/// use plating_systems::tags::Tag;
///
/// # #[cfg(all(feature = "cocoa", feature = "winui3"))]
/// type MyCustomTag = Tag<(WinUI3Definition, CocoaDefinition)>;
/// ```
/// ## Usage
/// Example
/// ```
/// # use plating_systems::tags::Tag;
/// # #[cfg(all(feature = "cocoa", feature = "winui3"))]
/// # use plating_systems::systems::{WinUI3Definition, CocoaDefinition};
/// # #[cfg(all(feature = "cocoa", feature = "winui3"))]
/// # type MyCustomTag = Tag<(WinUI3Definition, CocoaDefinition)>;
/// use plating_systems::types::ButtonAvailable;
/// struct SomeType {}
///
/// # #[cfg(all(feature = "cocoa", feature = "winui3"))]
/// // Something is only defined when Windows and Cocoa support Buttons.
/// // This is how the tag macro specifies which types are defined and which aren't.
/// type Something
/// where
///     MyCustomTag: ButtonAvailable,
/// = SomeType;
/// ```
/// ## See also
/// Plating ships with the following tags by default:
/// - [`AllTag`]
/// - [`MobileTag`]
/// - [`DesktopTag`]
/// as well as a [`Tier2Tag`] for less well supported platforms.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash)]
pub struct Tag<Sys>
where
    Sys: SystemsList + SystemDefinition,
{
    _sys: PhantomData<Sys>,
}

#[allow(rustdoc::missing_doc_code_examples)]
impl<Sys> TagTrait for Tag<Sys> where Sys: SystemsList + SystemDefinition {}


#[default_widgets]
#[tag(
    for (
         #[cfg(all(feature = "cocoa", target_os = "macos"))]
         crate::systems::Cocoa,
         #[cfg(all(feature = "winui3", target_os = "windows"))]
         crate::systems::WinUI3,
         #[cfg(all(feature = "qt", target_os = "linux"))]
         crate::systems::QT,
         #[cfg(all(feature = "gtk", target_os = "linux"))]
         crate::systems::GTK,
         #[cfg(all(feature = "android", target_os = "android"))]
         crate::systems::Android,
         #[cfg(all(feature = "ios", target_os = "ios"))]
         crate::systems::IOS,
         #[cfg(all(feature = "mock"))]
         crate::systems::Mock,
    )
)]
#[doc(notable_trait)]
pub trait All {}


#[default_widgets]
#[tag(
    for (
        #[cfg(all(feature = "mock"))]
        crate::systems::Mock,
    )
)]
/// A [`Tag`] representing the Tier 2 Systems that did not make it into the
/// other tags like [`All`], [`Desktop`], [`Mobile`].
///
/// Systems in this 'Tag' might be promoted in the future, but have a too
/// limited amount of widgets for now.
pub trait Tier2 {}

#[default_widgets]
#[tag(
    for (
        #[cfg(all(feature = "cocoa", target_os = "macos"))]
        crate::systems::Cocoa,
        #[cfg(all(feature = "winui3", target_os = "windows"))]
        crate::systems::WinUI3,
        #[cfg(all(feature = "qt", target_os = "linux"))]
        crate::systems::QT,
        #[cfg(all(feature = "gtk", target_os = "linux"))]
        crate::systems::GTK,
        #[cfg(all(feature = "mock"))]
        crate::systems::Mock,
    )
)]
/// A [`Tag`] representing the Desktop systems supported by default
/// by plating.
#[doc(notable_trait)]
pub trait Desktop {}

/*
#[doc(cfg(any(feature = "android",
              feature = "ios",
              feature = "mock",
)))]*/
#[default_widgets]
#[tag(
    for (
        #[cfg(all(feature = "android", target_os = "android"))]
        crate::systems::Android,
        #[cfg(all(feature = "ios", target_os = "ios"))]
        crate::systems::IOS,
        #[cfg(all(feature = "mock"))]
        crate::systems::Mock,
    )
)]
/// A [`Tag`] representing the Mobile systems supported by default
/// by plating.
#[doc(notable_trait)]
pub trait Mobile {}
