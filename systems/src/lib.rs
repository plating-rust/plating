//! Systems for the plating framework
//!
//! # Feature Flags
//! | Feature | Description | Default |
//! |:-------:|-------------|:-------:|
//! | mock | Enables the [Mock](`crate::systems::Mock`) System. If enabled, the
//! mock System is used as the [`Native`] System during testing. | Yes | | cocoa
//! | Enables the [Cocoa](`crate::systems::Cocoa`) System. If enabled, the cocoa
//! System is used as the [`Native`] System on Mac. | Yes | | winui3 | Enables
//! the [WinUI3](`crate::systems::WinUI3`) System. If enabled, the winui3 System
//! is used as the [`Native`] System on Windows. | Yes | | gtk | Enables the
//! [GTK](`crate::systems::GTK`) System. If enabled, the GTK System is used as
//! the [`Native`] System on Linux. | Yes | | qt | Enables the
//! [QT](`crate::systems::QT`) System. If enabled, the QT System is used as the
//! [`Native`] System on Linux. | No | | android | Enables the
//! [IOS](`crate::systems::IOS`) System. If enabled, the IOS System is used as
//! the [`Native`] System on IOS. | No | | ios | Enables the
//! [Android](`crate::systems::Android`) System. If enabled, the Android System
//! is used as the [`Native`] System on Android. | No | | serde | Enables the
//! 'serde::Serialize' and 'serde::Deserialize' traits for most structs. | Yes |
//! | native | Generates [`Native`] type. Disable if you want to define yourself
//! what System to use on what platform. | Yes |
//!
//! If 'GTK' and 'QT' are enabled at the same time, no [`Native`] will be
//! defined on Linux.
//!
//!
//! Big todo:

#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::missing_doc_code_examples,
    missing_debug_implementations,
    missing_copy_implementations,
    unused_import_braces,
    unused_crate_dependencies
)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(trivial_bounds)]
#![feature(inherent_associated_types)]
#![feature(doc_cfg)]
#![feature(doc_notable_trait)]

#[cfg(feature = "serde")]
extern crate serde;

pub use default_widgets::default_widgets;


/// Module containing traits for dealing with Systems,
/// as well as the actual System implementations.
pub mod systems;
/// A [Tag](`crate::tags::Tag`) is a list of Systems.
/// This module contains the Tags [All](`crate::tags::All`),
/// [Mobile](`crate::tags::Mobile`) and [Desktop](`crate::tags::Desktop`) as
/// well as required Traits.
pub mod tags;





/// Contains the 'HasFoo', 'FooAvailable' and 'AllHaveFoo' traits.
///
/// See [`marker`] on how to create your own.
pub mod types;

#[doc(inline)]
pub use plating_systems_macros::{build_widget_list, marker, tag};


#[cfg(doc)]
#[doc(hidden)]
pub struct PlatformDependant {}

#[cfg(doc)]
/// Type Alias for the System that is considered the default on the current
/// platform.
///
/// Defaults to
/// - [Cocoa](`crate::systems::Cocoa`) on Osx
/// - [Android](`crate::systems::Android`) on Android
/// - [IOS](`crate::systems::IOS`) on Ios
/// - [WinUI3](`crate::systems::WinUI3`) on Windows
/// - [GTK](`crate::systems::GTK`) or [QT](`crate::systems::QT`) on Linux,
///   depending on feature flags
/// - [Mock](`crate::systems::Mock`) during testing or when all other Systems
///   are not activated via feature flags. If Mock is not activated via feature
///   flags, will use the default backend even during testing.

/// # Availability
/// native is not available on unsupported platforms,
/// or when the default backend is not enabled via the appropriate feature flag.
///
/// Native is also not defined on Linux when both [GTK](`crate::systems::GTK`)
/// and [QT](`crate::systems::QT`) are activated via feature flags. TODO: find a
/// more satisfying solution.
pub type Native = PlatformDependant;

#[cfg(all(
    feature = "native",
    any(
        test,
        all(
            not(feature = "cocoa"),
            not(feature = "winui3"),
            not(feature = "ios"),
            not(feature = "android"),
            not(feature = "qt"),
            not(feature = "gtk"),
            not(doc),
        )
    ),
    feature = "mock"
))]
#[doc(hidden)]
pub type Native = systems::Mock;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "macos",
    feature = "cocoa",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::Cocoa;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "windows",
    feature = "winui3",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::WinUI3;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "linux",
    not(feature = "qt"),
    feature = "gtk",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::GTK;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "linux",
    not(feature = "gtk"),
    feature = "qt",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::QT;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "android",
    feature = "android",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::Android;
#[cfg(all(
    feature = "native",
    not(all(test, feature = "mock")),
    target_os = "ios",
    feature = "ios",
    not(doc),
))]
#[doc(hidden)]
pub type Native = systems::IOS;


#[cfg(all(
    feature = "native",
    not(feature = "mock"),
    not(feature = "cocoa"),
    not(feature = "winui3"),
    not(feature = "ios"),
    not(feature = "android"),
    not(feature = "qt"),
    not(feature = "gtk"),
    not(doc),
))]
compile_error!(
    "Native feature enabled but no System to define it to.\n
                Please enable at least one of the following features:
                    - mock
                    - cocoa
                    - winui3
                    - qt
                    - gtk
                    - android
                    - ios"
);


#[cfg(all(test, feature = "native"))]
#[cfg(any(
    feature = "mock",
    all(feature = "android", target_os = "android"),
    all(feature = "ios", target_os = "ios"),
    all(feature = "winui3", target_os = "windows"),
    all(feature = "cocoa", target_os = "macos"),
    all(feature = "gtk", not(feature = "qt"), target_os = "linux"),
    all(feature = "qt", not(feature = "gtk"), target_os = "linux"),
))]
mod tests {
    use plating_core::mock::MockButtonOutlet;
    use plating_core::prelude::*;

    use super::types::HasButton;
    //use super::setup::system::CocoaSpecific;
    use super::Native;
    use crate::tags::Desktop;
    #[test]
    fn it_works() {
        let _button_a = <Native as Desktop>::Button::new(MockButtonOutlet {});
        let _button_b = <Native as HasButton>::Button::new(MockButtonOutlet {});

        //let button_c = <Native as CocoaSpecific>::Button::new(5);
        /*#[cfg(any(
            all(feature = "android", target_os = "android"),
            all(feature = "ios", target_os = "ios"),
            all(feature = "mock", test)
        ))]
        let button_a = <Native as crate::tags::Mobile>::Tabs::new(5);*/
        //let b = <Native as Desktop>::Tabs::new(5);
    }
}

#[doc(hidden)]
#[cfg(feature = "native")]
#[cfg(any(
    all(feature = "mock", test),
    all(feature = "android", target_os = "android"),
    all(feature = "ios", target_os = "ios"),
    all(feature = "winui3", target_os = "windows"),
    all(feature = "cocoa", target_os = "macos"),
    all(feature = "gtk", not(feature = "qt"), target_os = "linux"),
    all(feature = "qt", not(feature = "gtk"), target_os = "linux"),
))]
pub fn it_works() -> plating_core::PlatingResult<()> {
    use plating_core::mock::{MockOnly, MockOnlyWidget};
    use plating_core::prelude::*;
    use plating_core::utils::children::children_list;

    use crate::tags::Desktop;
    use crate::types::HasButton;

    let button_a = <Native as Desktop>::Button::new(plating_core::mock::MockButtonOutlet {})?;
    let button_b = <Native as HasButton>::Button::new(plating_core::mock::MockButtonOutlet {})?;
    let mock_only = MockOnlyWidget::new(plating_core::mock::MockOnlyOutlet {})?;
    let _wind = <Native as Desktop>::Window::new(plating_core::mock::MockWindowOutlet {
        children: children_list!(button_a, button_b, mock_only),
        menu:     children_list!(),
    })?;
    //let button_c = <Native as CocoaSpecific>::Button::new(5);
    /*#[cfg(any(
        all(feature = "android", target_os = "android"),
        all(feature = "ios", target_os = "ios"),
        all(feature = "mock", test)
    ))]
    let button_a = <Native as Mobile>::Tabs::new(5);*/
    //let b = <Native as Desktop>::Tabs::new(5);
    Ok(())
}
