/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Connectable, Named};
use crate::widgets::{default_system, System, Widget};

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq, Hash)] //required in cached version
pub struct RootParameters {}

pub trait RootHandlerTrait {
    fn set_exit_handler(&mut self, handler: Box<impl FnMut()>);
    fn add_exit_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}

/// native Root Widgets need to implement this trait
///
/// #Requirements
/// Widgets implementing this trait, also need to implement NativeWidget as well
/// as OutletAdapter<RootChildren<S>>
pub trait NativeRoot<S: System>:
    Widget<S, PARAMS = S::RootParameterTye> + Outlet<RootChildren<S>, S> + RootHandlerTrait
{
    /// Calling this function starts the main loop.
    /// Only returns once the app is closed.
    fn run(&self) -> std::result::Result<(), anyhow::Error>;
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum RootChildren<S: System = default_system> {
    WINDOW(S::WindowType),
}

impl<S: System> Named for RootChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::WINDOW(window) => window.name(),
        }
    }
}

impl<S: System> Connectable for RootChildren<S> {
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

impl<S: System> Child<S::RootType, RootChildren<S>, S> for RootChildren<S> {
    fn adding_to_parent(
        &mut self,
        parent: &<S::RootType as Outlet<RootChildren<S>, S>>::ParentData,
    ) {
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
