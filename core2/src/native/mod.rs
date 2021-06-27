/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//unsafe code is allowed in the lowest level aka native level.
#![allow(unsafe_code)]


mod app;
mod menu;
mod window;

//Todo: figure out where to export to.
pub mod events;


mod native_widget;
pub use native_widget::{NativeBuilder, NativeWidget, NativeWidgetContainer};

mod native_child_of;
pub use native_child_of::NativeChildOf;

use crate::widget::Backend;

pub mod data;

pub trait Native<BACKEND: Backend> {}

impl<BACKEND: Backend> Native<BACKEND> for () {}


impl<BACKEND: Backend, Head, Tail> Native<BACKEND> for (Head, Tail)
where
    Head: Native<BACKEND>,
    Tail: Native<BACKEND> + crate::utils::SettingsList,
{
}

pub trait ToNative<BACKEND: Backend> {
    type Result: Native<BACKEND>;

    fn to_native(&self) -> Self::Result;
}

impl<BACKEND: Backend> ToNative<BACKEND> for () {
    type Result = ();

    fn to_native(&self) -> Self::Result {
        ()
    }
}

impl<BACKEND: Backend, Head, Tail> ToNative<BACKEND> for (Head, Tail)
where
    Head: ToNative<BACKEND>,
    Tail: ToNative<BACKEND> + crate::utils::SettingsList,
    <Tail as ToNative<BACKEND>>::Result: crate::utils::SettingsList,
{
    type Result = (
        <Head as ToNative<BACKEND>>::Result,
        <Tail as ToNative<BACKEND>>::Result,
    );

    fn to_native(&self) -> Self::Result {
        (self.0.to_native(), self.1.to_native())
    }
}

pub mod cocoa;

pub mod prelude {
    //nothing to prelude from this stage as this level is not cross-platform
}
