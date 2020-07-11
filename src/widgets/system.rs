/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::{
    button::{ButtonParameters, NativeButton},
    menu::{MenuParameters, NativeMenu},
    menu_item::{MenuItemParameters, NativeMenuItem},
    root::{NativeRoot, RootParameters},
    window::{NativeWindow, WindowParameters},
};

/// A System represents a GUI System (or you could also call it backend).
///
/// # Examples of Systems
/// - cocoa
/// - win32
/// - gtk
/// - qt
///
/// # Implementation
/// A System consists of mainly typedefs to the native widget implementations.
///
/// If you're just starting implement a new System, you should copy [`MockSystem`] and start replacing
/// one-by-one widget with actual implementations.
///
/// # Motivation
/// Contains
///     - typedefs for all native widget types
///     - typedefs for all parameters those native types take
/// # Usage
/// You probably never create or interact with a system except as a Template parameter.
/// ```ignore
/// Button<SomeImaginarySystem>::new(...)
/// ```
///
/// ## Typedefs
/// If you want to stay cross-platform it is a good idea to use [`default_system`](crate::widgets::default_system) as much as possible.
/// ```ignore
/// use plating::widgets::default_system;
///
/// Button<default_system>::new(...)
/// ````
pub trait System
where
    Self: std::fmt::Debug + Sized + std::fmt::Display,
{
    /// Returns the name of this system.
    fn name() -> &'static str;

    /// The internal handle used by this system.
    /// Could be a `pointer`, `id`or whatever. Using it directly usually means you loose cross-platform compatibility.
    type InternalHandle;

    type RootParameterTye: From<RootParameters>;
    type RootType: NativeRoot<Self>;

    type ButtonParameterType: From<ButtonParameters>;
    type ButtonType: NativeButton<Self>;

    type WindowParameterType: From<WindowParameters>;
    type WindowType: NativeWindow<Self>;

    type MenuParameterType: From<MenuParameters>;
    type MenuType: NativeMenu<Self>;

    type MenuItemParameterType: From<MenuItemParameters>;
    type MenuItemType: NativeMenuItem<Self>;
}
