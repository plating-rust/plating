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

pub mod delegates;
pub mod error;

pub use button::*;
pub use defs::*;
pub use menu::*;
pub use menu_item::*;
pub use root::*;
pub use window::*;

pub(self) mod utils;

use crate::widgets::System;
use delegates::CocoaWindowDelegate;

#[derive(Debug)]
pub struct CocoaSystem {}

impl System for CocoaSystem {
    fn name() -> &'static str {
        "cocoa"
    }

    type InternalHandle = CocoaDefaultHandleType;
    type ErrorType = error::CocoaError;

    type RootParameterTye = CocoaRootParameters;
    type RootType = CocoaRoot;

    type ButtonParameterType = CocoaButtonParameters;
    type ButtonType = CocoaButton;

    type WindowType = CocoaWindow;

    type WindowParameterType = CocoaWindowParameters;

    type MenuParameterType = CocoaMenuParameters;
    type MenuType = CocoaMenu;
    type MenuItemParameterType = CocoaMenuItemParameters;
    type MenuItemType = CocoaMenuItem;
}

impl std::fmt::Display for CocoaSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CocoaSystem::name())
    }
}
