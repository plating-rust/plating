/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::Identity;
use crate::events::{ListenerType, PermissionResult, PermissionState};
use crate::features::serde::{Deserialize, Serialize};
use std::fmt;

//todo: #[derive(Eq, PartialEq)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DetachEvent {}

pub trait DetachTopic {
    fn add_listener(
        &self,
        when: ListenerType,
        handler: Box<impl FnMut(&DetachEvent, &dyn Identity, &PermissionState)>,
    );
    fn set_handler(
        &self,
        handler: Box<impl FnMut(&DetachEvent, &dyn Identity) -> PermissionResult>,
    );
}

pub struct DetachChildEvent<'a> {
    child: &'a dyn Identity,
}
impl<'a> fmt::Debug for DetachChildEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttachChildEvent")
            .field("child", &self.child.id())
            .finish()
    }
}

pub trait DetachChildTopic {
    fn add_listener(
        when: ListenerType,
        handler: Box<impl FnMut(&DetachChildEvent, &dyn Identity, &PermissionState)>,
    );
    fn set_handler(handler: Box<impl FnMut(&DetachChildEvent, &dyn Identity) -> PermissionResult>);
}
