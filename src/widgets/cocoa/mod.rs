/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

mod defs;

mod button;
mod event;
mod menu;
mod menu_item;
mod root;
mod window;

pub mod error;

pub use button::*;
pub use defs::*;
pub use menu::*;
pub use menu_item::*;
pub use root::*;
pub use window::*;

pub(self) mod utils;

use crate::widgets::System;

#[derive(Debug)]
pub struct CocoaSystem {}

impl System for CocoaSystem {
    type InternalHandle = CocoaDefaultHandleType;
    type ErrorType = error::CocoaError;

    type RootParameterTye = CocoaRootParameters;
    /// Define NativeRoot to [CocoaRoot](crate::widgets::cocoa::CocoaRoot)
    type RootType = CocoaRoot;

    type ButtonParameterType = CocoaButtonParameters;
    /// Define NativeButton to [CocoaButton](crate::widgets::cocoa::CocoaButton)
    type ButtonType = CocoaButton;

    type WindowType = CocoaWindow;
    type WindowParameterType = CocoaWindowParameters;

    type MenuParameterType = CocoaMenuParameters;
    type MenuType = CocoaMenu;
    type MenuItemParameterType = CocoaMenuItemParameters;
    type MenuItemType = CocoaMenuItem;
}
