/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */


/// Contains typedefs to cocoa backend types
/// and other types used in the [`widgets::cocoa`](crate::widgets::cocoas) module
 
use cocoa::base::id;

/// The `CocoaDefaultHandleType` is set to [`cocoa::base::id`].
pub type CocoaDefaultHandleType = id;

/// The CocoaEventType used for callbacks in Cocoa Widgets. 
pub type CocoaEventType = super::event::CocoaEvent<CocoaDefaultHandleType>;

