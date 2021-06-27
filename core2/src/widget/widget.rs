/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::{Native, NativeBuilder, ToNative};
use crate::prelude::Backend;
use crate::utils::{OutletHolder, Property, SettingsList};
use crate::PlatingResult;

//TODO: find better name
pub trait WidgetAbstractionLevel {}

impl WidgetAbstractionLevel for () {}

impl<Head, Tail> WidgetAbstractionLevel for (Head, Tail)
where
    Head: WidgetAbstractionLevel,
    Tail: WidgetAbstractionLevel + SettingsList,
{
}


pub trait WidgetBuilder<STATE, BACKEND>
where
    Self: Sized,
    BACKEND: Backend,
{
    type OutletType: OutletHolder;

    // makro, requires CocoaWindowPropertyProvider
    fn new<SL>(
        settings: SL,
        outlet: <Self as WidgetBuilder<STATE, BACKEND>>::OutletType,
    ) -> PlatingResult<Self>
    where
        Self: NativeBuilder<STATE, Backend = BACKEND>,
        STATE: Default,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<BACKEND>,
        <SL as ToNative<BACKEND>>::Result:
            SettingsList + Native<BACKEND> + Property<STATE, Self, BACKEND>,
    {
        WidgetBuilder::new_with_state(STATE::default(), settings, outlet)
    }

    // makro, requires CocoaWindowPropertyProvider
    fn new_with_state<SL>(
        state: STATE,
        settings: SL,
        outlet: <Self as WidgetBuilder<STATE, BACKEND>>::OutletType,
    ) -> PlatingResult<Self>
    where
        Self: NativeBuilder<STATE, Backend = BACKEND>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<BACKEND>,
        <SL as ToNative<BACKEND>>::Result:
            SettingsList + Native<BACKEND> + Property<STATE, Self, BACKEND>;
}
