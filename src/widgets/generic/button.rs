/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::events::ListenerType;
use crate::widgets::utils::Child;
use crate::widgets::{System, Widget, WindowChildren};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ButtonParameters {
    pub label: Option<String>,
}

pub trait ButtonHandlerTrait {
    fn set_exit_handler(&mut self, handler: Box<impl FnMut()>);
    fn add_exit_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}

/// native Button widgets need to implement this trait
///
/// #Requirements
/// Widgets implementing this trait, also need to implement
/// - NativeWidget
/// - Child
pub trait NativeButton<S: System>:
    Widget<S, PARAMS = S::ButtonParameterType>
    + ButtonHandlerTrait
    + Child<S::WindowType, WindowChildren<S>, S>
{
}
