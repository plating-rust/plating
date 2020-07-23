/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module contains a generic [`Window`] adapter and a structure
//! for generic parameters that work across all OS's.

use crate::widgets::outlet::Outlet;
use crate::widgets::root::RootChildren;
use crate::widgets::utils::{Child, Connectable, Identity, Parameters};
use crate::widgets::{default_system, System, Widget};

pub trait WindowParameters: Parameters {
    /// Sets the title of the window
    fn label(&self) -> &Option<String>;

    fn set_label(&mut self, label: String) -> &mut Self;
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self;
    fn unset_label(&mut self) -> &mut Self;

    /// Makes the window resizable.
    fn resizable(&self) -> Option<bool>;

    fn set_resizable(&mut self, resizable: bool) -> &mut Self;
    fn set_resizable_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_resizable(&mut self) -> &mut Self;

    /// Makes the window closable.
    fn closable(&self) -> Option<bool>;

    fn set_closable(&mut self, closable: bool) -> &mut Self;
    fn set_closable_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_closable(&mut self) -> &mut Self;

    /// Makes the window miniaturizable
    fn miniaturizable(&self) -> Option<bool>;

    fn set_miniaturizable(&mut self, closable: bool) -> &mut Self;
    fn set_miniaturizable_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_miniaturizable(&mut self) -> &mut Self;

    /// Makes the window maximizable.
    /// (Title and Main menu will still be shown)
    fn maximizable(&self) -> Option<bool>;

    fn set_maximizable(&mut self, closable: bool) -> &mut Self;
    fn set_maximizable_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_maximizable(&mut self) -> &mut Self;
    /// Allow the window to be fullscreen
    ///
    /// (Title and main menu will not be shown)
    fn fullscreenable(&self) -> Option<bool>;

    fn set_fullscreenable(&mut self, closable: bool) -> &mut Self;
    fn set_fullscreenable_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_fullscreenable(&mut self) -> &mut Self;
}

pub trait WindowHandlerTrait<S: System + ?Sized> //:
//AttachTopic<S::RootType, S>
{
}

pub trait Window<S: System + ?Sized>:
    Widget<S, PARAMS = S::WindowParameterType>
    + Outlet<WindowChildren<S>, S>
    + Outlet<MainMenuChildren<S>, S>
    + WindowHandlerTrait<S>
    + Child<S::RootType, RootChildren<S>, S>
    + Default
{
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowChildren<S: System + ?Sized = default_system> {
    BUTTON(S::ButtonType),
}

impl<S: System + ?Sized> Connectable for WindowChildren<S> {
    fn connecting(&mut self) {
        match self {
            Self::BUTTON(button) => button.connecting(),
        }
    }

    fn disconnecting(&mut self) {
        match self {
            Self::BUTTON(button) => button.disconnecting(),
        }
    }

    fn connected(&self) -> bool {
        match self {
            Self::BUTTON(button) => button.connected(),
        }
    }
}

impl<S: System + ?Sized> Child<S::WindowType, WindowChildren<S>, S> for WindowChildren<S> {
    fn adding_to_parent(&mut self, parent: &<S::WindowType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::BUTTON(button) => button.adding_to_parent(parent),
        }
    }
    fn removing_from_parent(&mut self) {
        match self {
            Self::BUTTON(button) => button.removing_from_parent(),
        }
    }

    fn added(&self) -> bool {
        match self {
            Self::BUTTON(button) => button.added(),
        }
    }
}
/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System + ?Sized> Identity for WindowChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.id(),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren<S: System + ?Sized = default_system> {
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System + ?Sized> Identity for MainMenuChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.id(),
        }
    }
}

impl<S: System + ?Sized> Connectable for MainMenuChildren<S> {
    fn connecting(&mut self) {
        match self {
            Self::MENU(menu) => menu.connecting(),
        }
    }

    fn disconnecting(&mut self) {
        match self {
            Self::MENU(menu) => menu.disconnecting(),
        }
    }

    fn connected(&self) -> bool {
        match self {
            Self::MENU(menu) => menu.connected(),
        }
    }
}
impl<S: System + ?Sized> Child<S::WindowType, MainMenuChildren<S>, S> for MainMenuChildren<S> {
    fn adding_to_parent(&mut self, parent: &<S::WindowType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => <dyn Child<S::WindowType, Self, S>>::adding_to_parent(menu, parent),
        }
    }
    fn removing_from_parent(&mut self) {
        match self {
            Self::MENU(menu) => menu.removing_from_parent(),
        }
    }

    fn added(&self) -> bool {
        match self {
            Self::MENU(menu) => menu.added(),
        }
    }
}
