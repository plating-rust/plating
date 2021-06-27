/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

use std::pin::Pin;

use crate::backend::cocoa::StrongPtr;
use crate::utils::{ChildrenList, OutletHolder};

pub(crate) struct CocoaInternal<STATE, CHILDREN, OUTLET>
where
    CHILDREN: ChildrenList,
    OUTLET: OutletHolder,
{
    pub state:  STATE,
    pub outlet: OUTLET,

    pub native_handle: StrongPtr,

    _unpinned: std::marker::PhantomPinned,

    _phantom: std::marker::PhantomData<CHILDREN>,
}
impl<STATE, CHILDREN, OUTLET> CocoaInternal<STATE, CHILDREN, OUTLET>
where
    CHILDREN: ChildrenList,
    OUTLET: OutletHolder,
{
    pub fn new(
        handle: StrongPtr,
        state: STATE,
        outlet: OUTLET,
    ) -> Pin<Box<CocoaInternal<STATE, CHILDREN, OUTLET>>> {
        Box::pin(CocoaInternal {
            native_handle: handle,
            state,
            outlet,

            _unpinned: Default::default(),
            _phantom: Default::default(),
        })
    }
}

impl<STATE, CHILDREN, OUTLET> std::fmt::Debug for CocoaInternal<STATE, CHILDREN, OUTLET>
where
    CHILDREN: ChildrenList + std::fmt::Debug,
    STATE: std::fmt::Debug,
    OUTLET: OutletHolder + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ops::Deref;

        f.debug_struct("CocoaInternal")
            .field("state", &self.state)
            .field("outlet", &self.outlet)
            .field("native_handle", &self.native_handle.deref())
            .finish()
    }
}

pub use super::app::cocoa::{CocoaApp, CocoaAppOutlet, CocoaAppWidget};
pub use super::menu::cocoa::{CocoaMenu, CocoaMenuOutlet, CocoaMenuWidget};
pub use super::window::cocoa::{CocoaWindow, CocoaWindowOutlet, CocoaWindowWidget};
