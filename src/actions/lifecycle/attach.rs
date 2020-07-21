/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::Identity;
use crate::events::{ListenerType, PermissionResult, PermissionState};
use crate::widgets::{default_system, System, Widget};

use std::fmt;
use std::marker::PhantomData;

//todo: #[derive(Eq, PartialEq)]
pub struct AttachEvent<W: Widget<S>, S: System = default_system> {
    //todo: parent: &'a W,
    _s: PhantomData<S>,
    _w: PhantomData<W>,
}
impl<W: Widget<S>, S: System> AttachEvent<W, S> {
    pub fn new() -> Self {
        Self {
            _s: PhantomData,
            _w: PhantomData,
        }
    }
}
impl<W: Widget<S>, S: System> Clone for AttachEvent<W, S> {
    fn clone(&self) -> Self {
        Self {
            _s: self._s,
            _w: self._w,
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
        handler: Box<impl FnMut(&AttachEvent<W, S>, &dyn Identity) -> PermissionResult>,
    );
}

pub struct AttachChildEvent<'a> {
    child: &'a dyn Identity,
}
impl<'a> fmt::Debug for AttachChildEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttachChildEvent")
            .field("child", &self.child.id())
            .finish()
    }
}

pub trait AttachChildTopic {
    fn add_listener<'b>(
        when: ListenerType,
        handler: Box<impl FnMut(&'b AttachChildEvent, &'b dyn Identity, &'b PermissionState)>,
    );
    fn set_handler(handler: Box<impl FnMut(&AttachChildEvent, &dyn Identity) -> PermissionResult>);
}
