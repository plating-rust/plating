/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::Named;
use crate::events::{ListenerType, PermissionResult, PermissionState};
use crate::widgets::{default_system, System, Widget};

use std::fmt;
use std::marker::PhantomData;

//todo: #[derive(Eq, PartialEq)]
pub struct AttachEvent<W: Widget<S>, S: System = default_system> {
    //todo: parent: &'a W,
    _S: PhantomData<S>,
    _W: PhantomData<W>,
}
impl<W: Widget<S>, S: System> AttachEvent<W, S> {
    pub fn new() -> Self {
        Self {
            _S: PhantomData,
            _W: PhantomData,
        }
    }
}
impl<W: Widget<S>, S: System> Clone for AttachEvent<W, S> {
    fn clone(&self) -> Self {
        AttachEvent {
            _S: self._S,
            _W: self._W,
        }
    }
}
impl<W: Widget<S>, S: System> fmt::Debug for AttachEvent<W, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttachEvent")
            //todo: .field("parent", &self.parent.name())
            .finish()
    }
}

pub trait AttachTopic<W: Widget<S>, S: System = default_system> {
    fn observe(&self, when: ListenerType);
    fn set_handler(
        &self,
        handler: Box<impl FnMut(&AttachEvent<W, S>, &dyn Named) -> PermissionResult>,
    );
}

pub struct AttachChildEvent<'a> {
    child: &'a dyn Named,
}
impl<'a> fmt::Debug for AttachChildEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttachChildEvent")
            .field("child", &self.child.name())
            .finish()
    }
}

pub trait AttachChildTopic {
    fn add_listener<'b>(
        when: ListenerType,
        handler: Box<impl FnMut(&'b AttachChildEvent, &'b dyn Named, &'b PermissionState)>,
    );
    fn set_handler(handler: Box<impl FnMut(&AttachChildEvent, &dyn Named) -> PermissionResult>);
}
