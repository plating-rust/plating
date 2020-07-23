/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Connectable, Identity, Parameters};
use crate::widgets::{default_system, System, Widget};

pub trait RootParameters: Parameters {}

pub trait RootHandlerTrait {
    fn set_exit_handler(&mut self, handler: Box<impl FnMut()>);
    fn add_exit_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}

/// native Root Widgets need to implement this trait
///
/// #Requirements
/// Widgets implementing this trait, also need to implement NativeWidget as well
/// as OutletAdapter<RootChildren<S>>
pub trait Root<S: System + ?Sized>:
    Widget<S, PARAMS = S::RootParameterType> + Outlet<RootChildren<S>, S> + RootHandlerTrait + Default
{
    /// Calling this function starts the main loop.
    /// Only returns once the app is closed.
    fn run(&self) -> std::result::Result<(), anyhow::Error>;
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum RootChildren<S: System + ?Sized = default_system> {
    WINDOW(S::WindowType),
}

impl<S: System + ?Sized> Identity for RootChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::WINDOW(window) => window.id(),
        }
    }
}

impl<S: System + ?Sized> Connectable for RootChildren<S> {
    fn connecting(&mut self) {
        match self {
            Self::WINDOW(window) => window.connecting(),
        }
    }

    fn disconnecting(&mut self) {
        match self {
            Self::WINDOW(window) => window.disconnecting(),
        }
    }

    fn connected(&self) -> bool {
        match self {
            Self::WINDOW(window) => window.connected(),
        }
    }
}

impl<S: System + ?Sized> Child<S::RootType, RootChildren<S>, S> for RootChildren<S> {
    fn adding_to_parent(&mut self, parent: &<S::RootType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::WINDOW(window) => window.adding_to_parent(parent),
        }
    }
    fn removing_from_parent(&mut self) {
        match self {
            Self::WINDOW(window) => window.removing_from_parent(),
        }
    }

    fn added(&self) -> bool {
        match self {
            Self::WINDOW(window) => window.added(),
        }
    }
}
