/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Named};
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
    fn run(&self) -> std::result::Result<(), S::ErrorType>;
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
impl<S: System> Child<S::RootType, RootChildren<S>, S> for RootChildren<S> {
    fn adding_to(&self, parent: &<S::RootType as Outlet<RootChildren<S>, S>>::ParentData) {
        match self {
            Self::WINDOW(button) => button.adding_to(parent),
        }
    }
}
