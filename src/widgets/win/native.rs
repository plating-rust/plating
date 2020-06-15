/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Native typedefs for the windows OS.
//! 
//! Most of the typedefs in here are for Widgets in [widgets::win](crate::widgets::win)
//! 
//! See here for other platform defines:
//! - MacOs: [widgets::cocoa::native](crate::widgets::cocoa::native)

/// todo: Define NativeRoot to [CocoaRoot](crate::widgets::win::WinRoot)
pub type NativeRoot = crate::widgets::cocoa::CocoaRoot;
/// todo: Define NativeRootParameters to [CocoaRootParameters](crate::widgets::win::WinRootParameters)
pub type NativeRootParameters = crate::widgets::cocoa::CocoaRootParameters;

/// todo: Define NativeButton to [CocoaButton](crate::widgets::win::CocoaButton)
pub type NativeButton = crate::widgets::cocoa::CocoaButton;
/// todo: Define NativeButtonParameters to [CocoaButtonParameters](crate::widgets::win::WinButtonParameters)
pub type NativeButtonParameters = crate::widgets::cocoa::CocoaButtonParameters;

/// todo: Define NativeWindow to [CocoaWindow](crate::widgets::win::WinWindow)
pub type NativeWindow = crate::widgets::cocoa::CocoaWindow;
/// todo: Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::win::WinWindowParameters)
pub type NativeWindowParameters = crate::widgets::cocoa::CocoaWindowParameters;

/// todo: Define NativeWindow to [CocoaWindow](crate::widgets::cocoa::CocoaWindow)
pub type NativeMenu = crate::widgets::cocoa::CocoaMenu;
/// todo: Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::cocoa::CocoaWindowParameters)
pub type NativeMenuParameters = crate::widgets::cocoa::CocoaMenuParameters;

/// todo: Define NativeWindow to [CocoaWindow](crate::widgets::cocoa::CocoaWindow)
pub type NativeMenuItem = crate::widgets::cocoa::CocoaMenuItem;
/// todo: Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::cocoa::CocoaWindowParameters)
pub type NativeMenuItemParameters = crate::widgets::cocoa::CocoaMenuItemParameters;

/// todo: Defined to [WinDefaultHandleType](super::WinDefaultHandleType)
pub type NativeDefaultHandleType = u32;


/// Defined to [WinError](super::error::WinError)
pub type NativeError = super::error::WinError;
/// Defined to [WinErrorKind](super::error::WinErrorKind)
pub type NativeErrorKind = super::error::WinErrorKind;