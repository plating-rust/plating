/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains traits for the native widgets to implement

use crate::widgets::{
    generic::WindowHandlerTrait, Child, MainMenuChildren, MenuChildren, NativeWidget,
    OutletAdapter, RootChildren, System, WindowChildren,
};

/// native Root Widgets need to implement this trait
///
/// #Requirements
/// Widgets implementing this trait, also need to implement NativeWidget as well
/// as OutletAdapter<RootChildren<S>>
pub trait NativeRoot<S: System>:
    NativeWidget<S, PARAMS = S::RootParameterTye> + OutletAdapter<RootChildren<S>, S>
{
    /// Calling this function starts the main loop.
    /// Only returns when the app is closed.
    fn run(&self) -> std::result::Result<(), S::ErrorType>;
}

/// native Button widgets need to implement this trait
///
/// #Requirements
/// Widgets implementing this trait, also need to implement
/// - NativeWidget
/// - Child
pub trait NativeButton<S: System>:
    NativeWidget<S, PARAMS = S::ButtonParameterType> + Child<S::WindowType, WindowChildren<S>, S>
{
}

pub trait NativeWindow<S: System>:
    NativeWidget<S, PARAMS = S::WindowParameterType>
    + OutletAdapter<WindowChildren<S>, S>
    + OutletAdapter<MainMenuChildren<S>, S>
    + WindowHandlerTrait
    + Child<S::RootType, RootChildren<S>, S>
{
}

pub trait NativeMenu<S: System>:
    NativeWidget<S, PARAMS = S::MenuParameterType>
    + OutletAdapter<MenuChildren<S>, S>
    + Child<S::MenuType, MenuChildren<S>, S>
    + Child<S::WindowType, MainMenuChildren<S>, S>
{
}
