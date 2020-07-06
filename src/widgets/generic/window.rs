/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module contains a generic [`Window`] adapter and a structure
//! for generic parameters that work across all OS's.

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::events::ListenerType;
use crate::widgets::outlet::Outlet;
use crate::widgets::root::RootChildren;
use crate::widgets::utils::{Child, Named};
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

pub trait WindowHandlerTrait {
    fn set_resize_handler(&mut self, handler: Box<impl FnMut()>);
    fn add_resize_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}

pub trait NativeWindow<S: System>:
    Widget<S, PARAMS = S::WindowParameterType>
    + Outlet<WindowChildren<S>, S>
    + Outlet<MainMenuChildren<S>, S>
    + WindowHandlerTrait
    + Child<S::RootType, RootChildren<S>, S>
{
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowChildren<S: System = default_system> {
    BUTTON(S::ButtonType),
}

impl<S: System> Child<S::WindowType, WindowChildren<S>, S> for WindowChildren<S> {
    fn adding_to(&self, parent: &<S::WindowType as Outlet<WindowChildren<S>, S>>::ParentData) {
        match self {
            Self::BUTTON(button) => button.adding_to(parent),
        }
    }
}
/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> Named for WindowChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren<S: System = default_system> {
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> Named for MainMenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
        }
    }
}
impl<S: System> Child<S::WindowType, MainMenuChildren<S>, S> for MainMenuChildren<S> {
    fn adding_to(&self, parent: &<S::WindowType as Outlet<MainMenuChildren<S>, S>>::ParentData) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::WindowType, MainMenuChildren<S>, S>>::adding_to(menu, parent)
            }
        }
    }
}
