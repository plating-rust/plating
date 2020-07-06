/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains the generic definitions for what widgets type can the children of a given type.
//!

use crate::widgets::{
    default_system, Child, GenericWidget, NativeWidget, OutletAdapter, System, WidgetHolder,
};

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonChildren<S: System = default_system> {
    BUTTON(S::ButtonType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for ButtonChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowChildren<S: System = default_system> {
    BUTTON(S::ButtonType),
}

impl<S: System> Child<S::WindowType, WindowChildren<S>, S> for WindowChildren<S> {
    fn adding_to(
        &self,
        parent: &<S::WindowType as OutletAdapter<WindowChildren<S>, S>>::ParentData,
    ) {
        match self {
            Self::BUTTON(button) => button.adding_to(parent),
        }
    }
}
/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for WindowChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum RootChildren<S: System = default_system> {
    WINDOW(S::WindowType),
}

impl<S: System> WidgetHolder for RootChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::WINDOW(window) => window.name(),
        }
    }
}
impl<S: System> Child<S::RootType, RootChildren<S>, S> for RootChildren<S> {
    fn adding_to(&self, parent: &<S::RootType as OutletAdapter<RootChildren<S>, S>>::ParentData) {
        match self {
            Self::WINDOW(button) => button.adding_to(parent),
        }
    }
}

// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum MenuChildren<S: System = default_system> {
    ITEM(S::MenuItemType), //todo
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for MenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
            Self::ITEM(item) => item.name(),
        }
    }
}
impl<S: System> Child<S::MenuType, MenuChildren<S>, S> for MenuChildren<S> {
    fn adding_to(&self, parent: &<S::MenuType as OutletAdapter<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::MenuType, MenuChildren<S>, S>>::adding_to(menu, parent)
            }
            Self::ITEM(item) => item.adding_to(parent),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren<S: System = default_system> {
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for MainMenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
        }
    }
}
impl<S: System> Child<S::WindowType, MainMenuChildren<S>, S> for MainMenuChildren<S> {
    fn adding_to(
        &self,
        parent: &<S::WindowType as OutletAdapter<MainMenuChildren<S>, S>>::ParentData,
    ) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::WindowType, MainMenuChildren<S>, S>>::adding_to(menu, parent)
            }
        }
    }
}
