/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::data::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::RootChildren;
use crate::widgets::{NativeWidget, OutletAdapter, System};

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)] //required in cached version
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
    NativeWidget<S, PARAMS = S::RootParameterTye> + OutletAdapter<RootChildren<S>, S> + RootHandlerTrait
{
    /// Calling this function starts the main loop.
    /// Only returns once the app is closed.
    fn run(&self) -> std::result::Result<(), S::ErrorType>;
}
