/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::widgets::utils::{Child, Identity, Parameters};
use crate::widgets::window::WindowChildren;
use crate::widgets::{default_system, System, Widget};

pub trait ButtonParameters: Parameters {
    fn label(&self) -> &Option<String>;

    fn set_label(&mut self, label: String) -> &mut Self;
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self;
    fn unset_label(&mut self) -> &mut Self;
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
pub trait Button<S: System>:
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
impl<S: System> Identity for ButtonChildren<S> {
    fn id(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.id(),
        }
    }
}
