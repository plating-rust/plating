/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module contains a generic [`Window`] adapter and a structure
//! for generic parameters that work across all OS's.

use crate::widgets::outlet::Outlet;
use crate::widgets::root::RootChildren;
use crate::widgets::utils::{Child, Connectable, Identity};
use crate::widgets::{default_system, System, Widget};

/// Generic parameters for creating and customizing Windows
///
/// All fields are optional and will either use OS Default values or sensible
/// custom default values where appropriate. Check Documentation of native Window Parameters for more details.
/// - [`CocoaWindowParameters`](crate::widget::cocoa::CocoaWindowParameters)
///
/// The above native window parameter struct implement the `From` trait to
/// get os specific parameters from this.
/// ```rust
/// use plating::widgets::window::WindowParameters;
///
/// let params = WindowParameters::default();
///
/// #[cfg(target_os = "macos")]
/// let native: plating::widgets::cocoa::CocoaWindowParameters = params.into();
/// ```
///
/// You cannot generate a WindowParameter from a native Parameter struct, because they have more information that might be lost.
#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq, Hash)] //required in cached version
pub struct WindowParameters {
    /// Sets the Position and Size of the window
    pub rect: Option<crate::Rect>,
    /// Sets the title of the window
    pub title: Option<String>,

    /// Makes the window resizable.
    pub resizable: Option<bool>,
    /// Makes the window closable.
    pub closable: Option<bool>,
    /// Makes the window miniaturizable
    pub miniaturizable: Option<bool>,
    /// Makes the window maximizable.
    /// (Title and Main menu will still be shown)
    pub maximizable: Option<bool>,
    /// Allow the window to be fullscreen
    ///
    /// (Title and main menu will not be shown)
    pub fullscreenable: Option<bool>,
}

pub trait WindowHandlerTrait<S: System> //:
//AttachTopic<S::RootType, S>
{
}

pub trait Window<S: System>:
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
pub enum WindowChildren<S: System = default_system> {
    BUTTON(S::ButtonType),
}

impl<S: System> Connectable for WindowChildren<S> {
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

impl<S: System> Child<S::WindowType, WindowChildren<S>, S> for WindowChildren<S> {
    fn adding_to_parent(
        &mut self,
        parent: &<S::WindowType as Outlet<WindowChildren<S>, S>>::ParentData,
    ) {
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
impl<S: System> Identity for WindowChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.id(),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren<S: System = default_system> {
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> Identity for MainMenuChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.id(),
        }
    }
}

impl<S: System> Connectable for MainMenuChildren<S> {
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
impl<S: System> Child<S::WindowType, MainMenuChildren<S>, S> for MainMenuChildren<S> {
    fn adding_to_parent(
        &mut self,
        parent: &<S::WindowType as Outlet<MainMenuChildren<S>, S>>::ParentData,
    ) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::WindowType, MainMenuChildren<S>, S>>::adding_to_parent(menu, parent)
            }
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
