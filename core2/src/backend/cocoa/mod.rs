/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

//! Contains typedefs to cocoa backend types
//! and other types used in the [`widgets::cocoa`](crate::widgets::cocoas)
//! module

pub use objc::rc::StrongPtr;

pub mod base {
    pub use cocoa::base::{id, nil, BOOL, NO, YES};
}

mod ns_string;


pub use core_foundation;


pub mod foundation {
    pub use cocoa::foundation::{
        NSAutoreleasePool,
        NSComparisonResult,
        NSPoint,
        NSRect,
        NSRunLoop,
        NSSize,
    };

    pub use super::ns_string::NSString;
}

pub mod appkit {
    pub use cocoa::appkit::{
        NSApp,
        NSApplication,
        NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular,
        NSBackingStoreBuffered,
        NSMenu,
        NSMenuItem,
        NSRunningApplication,
        NSWindow,
        NSWindowStyleMask,
    };
}
