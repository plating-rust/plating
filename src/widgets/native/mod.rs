/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

/// Typedef to the native widgets
///
/// Uses
/// - [widgets::cocoa::native](crate::widgets::cocoa::native) on osx
/// - [widgets::win::native](crate::widgets::win::native) on win
///
/// If you enable the feature `mock_os` it will use [widgets::mock::native](crate::widgets::mock::native) regardless ofs platform.
#[cfg(target_os = "macos")]
#[doc(cfg(target_os = "macos"))]
pub use crate::widgets::cocoa::CocoaSystem as System;
#[cfg(target_os = "windows")]
#[doc(cfg(target_os = "windows"))]
pub use crate::widgets::win::WinSystem as System;

pub mod traits;
