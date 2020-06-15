/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

/// Define NativeRoot to [CocoaRoot](crate::widgets::cocoa::CocoaRoot)
pub type NativeRoot = crate::widgets::cocoa::CocoaRoot;
/// Define NativeRootParameters to [CocoaRootParameters](crate::widgets::cocoa::CocoaRootParameters)
pub type NativeRootParameters = crate::widgets::cocoa::CocoaRootParameters;

/// Define NativeButton to [CocoaButton](crate::widgets::cocoa::CocoaButton)
pub type NativeButton = crate::widgets::cocoa::CocoaButton;
/// Define NativeButtonParameters to [CocoaButtonParameters](crate::widgets::cocoa::CocoaButtonParameters)
pub type NativeButtonParameters = crate::widgets::cocoa::CocoaButtonParameters;

/// Define NativeWindow to [CocoaWindow](crate::widgets::cocoa::CocoaWindow)
pub type NativeWindow = crate::widgets::cocoa::CocoaWindow;
/// Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::cocoa::CocoaWindowParameters)
pub type NativeWindowParameters = crate::widgets::cocoa::CocoaWindowParameters;

/// Define NativeWindow to [CocoaWindow](crate::widgets::cocoa::CocoaWindow)
pub type NativeMenu = crate::widgets::cocoa::CocoaMenu;
/// Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::cocoa::CocoaWindowParameters)
pub type NativeMenuParameters = crate::widgets::cocoa::CocoaMenuParameters;

/// Define NativeWindow to [CocoaWindow](crate::widgets::cocoa::CocoaWindow)
pub type NativeMenuItem = crate::widgets::cocoa::CocoaMenuItem;
/// Define NativeWindowParameters to [CocoaWindowParameters](crate::widgets::cocoa::CocoaWindowParameters)
pub type NativeMenuItemParameters = crate::widgets::cocoa::CocoaMenuItemParameters;

/// Mock id data type
pub type NativeDefaultHandleType = u64;

/// Defined to [MockError](super::error::MockError)
pub type NativeError = super::error::MockError;
/// Defined to [MockErrorKind](super::error::MockErrorKind)
pub type NativeErrorKind = super::error::MockErrorKind;