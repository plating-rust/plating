/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains *native* and *generic* widgets.
//!
//! All widgets can be categorized into:
//! - Actual **Backend** implementations.<br>
//!   They try to mimic the API of the os (within reason). That means using them is not cross-platform.<br>
//!   Each platform has their widgets in their own module.
//!     - [cocoa] for macos. *widget names start with Cocoa*. E.g. [CocoaWindow](cocoa::CocoaWindow)
//!     - [win] for windows. *widget names start with Win*. E.g. [WinWindow](win::WinWindow)
//!     - [native] All widgets for the current platform also have a typedef in the native module.
//! - **[generic]**: The widgets provide a cross-platform interface to the native widgets. They have smaller API to only expose features available in all Operating Systems.
//! - **[mock]**: They mimic the
//! All types can be mixed and matched. That means you can build 90% of your GUI with generic widgets and only sprinkle
//! in the native ones where more control or tighter is integration is wanted.
//!
//! ```rust
//! use plating::prelude::*;
//! use plating::widgets::generic::{Root, RootParameters, Window, WindowParameters};
//! #[cfg(target_os = "macos")]
//! use plating::widgets::cocoa::{CocoaButton, CocoaButtonParameters};
//! #[cfg(target_os = "win")]
//! use plating::widgets::win::{WinButton, WinButtonParameters};
//!
//! fn main() {
//!     // create a *generic* root element
//!     let mut root = Root::new(RootParameters::default()).unwrap();
//!     // create a *generic* window element
//!     let mut window = Window::new(WindowParameters::default()).unwrap();
//!
//!     // create a *native* element for more control
//!     #[cfg(target_os = "macos")]
//!     let mut button: CocoaButton = CocoaButton::new(CocoaButtonParameters::default()).unwrap();
//!     #[cfg(target_os = "win")]
//!     let mut button: WinButton = WinButton::new(WinButtonParameters::default()).unwrap();
//!
//!     //mix them together
//!     window.add_child(button);
//!     root.add_child(window);
//! }
//! ```
//!
//! This module contains several traits that are common to several different widget types.
//! For basic usage of `plating` it should be enough to import them via `use plating::prelude::*`

mod children;
pub mod generic;
pub mod mock;
mod outlet;
mod traits;

#[cfg(any(target_os = "macos", doc))]
#[doc(cfg(target_os = "macos"))]
pub mod cocoa;

#[cfg(any(target_os = "windows", doc))]
#[doc(cfg(target_os = "windows"))]
pub mod win;

//todo: pub mod linux;
//todo: pub mod android;
//todo: pub mod ios;

pub use children::*;
pub use outlet::*;
pub use traits::*;

/// Typedef to the native widgets
///
/// Uses
/// - [widgets::cocoa::native](crate::widgets::cocoa::native) on osx
/// - [widgets::win::native](crate::widgets::win::native) on win
///
/// If you enable the feature `mock_os` it will use [widgets::mock::native](crate::widgets::mock::native) regardless ofs platform.
pub mod native {
    #[cfg(target_os = "macos")]
    #[doc(cfg(target_os = "macos"))]
    pub use crate::widgets::cocoa::CocoaSystem as System;
    #[cfg(target_os = "windows")]
    #[doc(cfg(target_os = "windows"))]
    pub use crate::widgets::win::WinSystem as System;
}

//todo: #[cfg(any(feature="mock_os", test))]
//#[doc(cfg(feature="mock_os"))]
//pub use crate::widgets::mock::MockSystem as default_system;
//#[cfg(all(not(feature="mock_os"), not(test)))]
//#[doc(cfg(all(not(feature="mock_os"), not(test))))]
pub use crate::widgets::native::System as default_system;
