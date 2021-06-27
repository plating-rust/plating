/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use super::Native;
use crate::native::NativeWidget;
use crate::prelude::Backend;
use crate::utils::ChildOf;

pub trait NativeChildOf<T, TState, BACKEND>: Sized
where
    BACKEND: Backend,
    T: NativeWidget<TState, Backend = BACKEND>,
{
    fn setup(&self, parent: &T);
    fn connect(&self, parent: &T);
    fn disconnect(&self);
}

impl<T, Y, YState, BACKEND> NativeChildOf<Y, YState, BACKEND> for T
where
    T: ChildOf<Y> + Native<BACKEND>,
    Y: NativeWidget<YState, Backend = BACKEND>,
    BACKEND: Backend,
{
    fn setup(&self, parent: &Y) {
        <Self as ChildOf<Y>>::connect(self, parent);
    }

    fn connect(&self, parent: &Y) {
        <Self as ChildOf<Y>>::connect(self, parent);
    }

    fn disconnect(&self) {
        <Self as ChildOf<Y>>::disconnect(self);
    }
}
/*
impl<T, Y, TState, YState, ChildrenT, SYSTEM> NativeChildOf<Y, YState, ChildrenT, SYSTEM> for T
where
    T: ChildOf<Y> + NativeWidget<TState, System = SYSTEM>,
    Y: NativeWidget<YState, System = SYSTEM>,
    ChildrenT: ChildrenList,
    SYSTEM: System,
{
    fn connect(&self, parent: &Y) {
        <Self as ChildOf<Y>>::connect(self, parent);
    }
    fn disconnect(&self) {
        <Self as ChildOf<Y>>::disconnect(self);
    }
}*/
