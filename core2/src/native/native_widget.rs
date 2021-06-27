/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::Native;
use crate::prelude::Backend;
use crate::utils::{OutletHolder, Property, SettingsList};
use crate::PlatingResult;

pub trait NativeBuilder<STATE>
where
    Self: NativeWidget<STATE>,
{
    type OutletType: OutletHolder;

    fn new<SL>(settings: &SL, outlet: Self::OutletType) -> PlatingResult<Self>
    where
        //todo: reintroduce where appropriate
        //      CHILDREN: NativeChildOf<Self, STATE, Self::System>,
        STATE: Default,
        SL: SettingsList + Property<STATE, Self, Self::Backend> + Native<Self::Backend>,
    {
        Self::new_with_state(STATE::default(), settings, outlet)
    }

    fn new_with_state<SL>(
        state: STATE,
        settings: &SL,
        outlet: Self::OutletType,
    ) -> PlatingResult<Self>
    where
        //     CHILDREN: NativeChildOf<Self, STATE, Self::Backend>,
        SL: SettingsList + Property<STATE, Self, Self::Backend> + Native<Self::Backend>;
}

//TODO: rename and move to widget
pub trait NativeWidget<STATE>
where
    Self: Sized,
{
    type Backend: Backend;
    type InternalHandle;

    //TODO:  have options without children for CHILDREN == ()

    //TODO: get_children() when CHILDEN != ()

    // makro, requires CocoaWindowPropertyProvider
    fn apply<SL>(&mut self, settings: &SL) -> PlatingResult<()>
    where
        SL: SettingsList + Property<STATE, Self, Self::Backend> + Native<Self::Backend>;
}


pub trait NativeWidgetContainer<STATE, BACKEND>
where
    BACKEND: Backend,
    Self: Sized,
{
    fn apply<SL>(&mut self, settings: &SL) -> PlatingResult<()>
    where
        SL: SettingsList + Property<STATE, Self, BACKEND> + Native<BACKEND>;
}


impl<STATE, BACKEND> NativeWidgetContainer<STATE, BACKEND> for ()
where
    BACKEND: Backend,
{
    fn apply<SL>(&mut self, _settings: &SL) -> PlatingResult<()>
    where
        SL: SettingsList + Property<STATE, Self, BACKEND> + Native<BACKEND>,
    {
        //todo: if SL is != () this is illegal!!!
        Ok(())
    }
}
