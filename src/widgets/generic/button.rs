/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::lifecycle::AttachTopic;
use crate::events::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::utils::{Child, Named};
use crate::widgets::window::WindowChildren;
use crate::widgets::{default_system, System, Widget};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ButtonParameters {
    pub label: Option<String>,
}

pub trait ButtonHandlerTrait<S: System> //:
//    AttachTopic<S::Window, S>
{
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
    + ButtonHandlerTrait<S>
    + Child<S::WindowType, WindowChildren<S>, S>
{
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonChildren<S: System = default_system> {
    BUTTON(S::ButtonType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> Named for ButtonChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}
