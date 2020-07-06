/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

/// Enum representing the EventState after a Event Callback was called.
///
/// # Example
/// todo: example callback return handled and unhandled on some condition
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum EventState {
    /// represent that the event was handled and no further event handlers should be called
    HANDLED,
    /// this event has not yet been handled.
    UNHANDLED,
}

//todo: decide where to put
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ListenerType {
    Before,
    After,
}

/// Callback type definition.
///
/// Callback handlers must adhere to this type definition.
pub type Callback<T, W, E = ()> = dyn FnMut(&T, &mut W) -> Result<EventState, E>;

trait LifecycleHandler
where
    Self: Sized + std::fmt::Debug,
{
    fn add_create_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);

    fn add_display_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);

    fn add_destroy_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}
