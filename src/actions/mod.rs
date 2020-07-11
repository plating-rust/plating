/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub use crate::events::{HandledState, ListenerType};
/// Is the same ListenerType as in the events module just re-exported in the actions module
/// for your convenience
pub use crate::widgets::utils::Named;

pub mod lifecycle;

pub trait CustomTopic<T> {
    fn add_listener<'a>(
        when: ListenerType,
        handler: Box<impl FnMut(&T, &'a dyn Named, HandledState)>,
    );
    fn set_handler(handler: Box<impl FnMut(&T, &dyn Named)>);
    //todo: fn to_stream(when: ListenerType)
}
