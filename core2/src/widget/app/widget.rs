/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::cocoa::{CocoaApp, CocoaAppWidget};
use crate::native::{Native, NativeWidget, ToNative};
use crate::prelude::WidgetBuilder;
use crate::utils::{
    ChildOf,
    ChildrenList,
    ChildrenOutlet,
    Outlet,
    OutletHolder,
    Property,
    SettingsList,
};
use crate::widget::properties::Label;
use crate::widget::system::cocoa::Cocoa;
use crate::widget::{Backend, WidgetAbstractionLevel};
use crate::PlatingResult;

#[derive(Debug, Default)]
pub struct AppOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    pub children: CHILDREN,
}

impl<CHILDREN> OutletHolder for AppOutlet<CHILDREN> where CHILDREN: ChildrenList {}

impl<CHILDREN> Outlet<ChildrenOutlet> for AppOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    type Children = CHILDREN;

    fn get(&self) -> &Self::Children {
        &self.children
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}

pub trait AppWidget<STATE, CHILDREN, BACKEND>
where
    CHILDREN: ChildrenList,
    Self: Sized,
    BACKEND: Backend,
{
    // macro, requires CocoaWindowPropertyProvider
    fn apply<SL>(&mut self, settings: SL) -> PlatingResult<()>
    where
        Self: NativeWidget<STATE, Backend = BACKEND>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<BACKEND>,
        <SL as ToNative<BACKEND>>::Result:
            SettingsList + Native<BACKEND> + Property<STATE, Self, BACKEND>;

    //custom property settings
    fn set_label(&mut self, label: &Label) -> PlatingResult<()>;

    //custom logic
    fn run(&mut self) -> PlatingResult<()>
    where
        CHILDREN: ChildOf<Self> + Native<BACKEND>;
}

impl<STATE, CHILDREN> WidgetBuilder<STATE, Cocoa> for CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList + ChildOf<Self> + Native<Cocoa>,
{
    type OutletType = AppOutlet<CHILDREN>;

    fn new_with_state<SL>(
        state: STATE,
        settings: SL,
        outlet: Self::OutletType,
    ) -> PlatingResult<Self>
    where
        Self: NativeWidget<STATE, Backend = Cocoa>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<Cocoa>,
        <SL as ToNative<Cocoa>>::Result: SettingsList
            + Native<Cocoa>
            + Property<STATE, Self, <Self as NativeWidget<STATE>>::Backend>,
    {
        <Self as crate::native::NativeBuilder<_>>::new_with_state(
            state,
            &settings.to_native(),
            outlet.into(),
        )
    }
}

impl<STATE, CHILDREN> AppWidget<STATE, CHILDREN, Cocoa> for CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
    Self: NativeWidget<STATE, Backend = Cocoa>,
{
    fn apply<SL>(&mut self, settings: SL) -> PlatingResult<()>
    where
        Self: NativeWidget<STATE>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<Cocoa>,
        <SL as ToNative<Cocoa>>::Result: SettingsList
            + Native<<Self as NativeWidget<STATE>>::Backend>
            + Property<STATE, Self, <Self as NativeWidget<STATE>>::Backend>,
    {
        <Self as NativeWidget<STATE>>::apply(self, &settings.to_native())
    }

    fn set_label(&mut self, label: &Label) -> PlatingResult<()> {
        <Self as CocoaApp<STATE, CHILDREN>>::set_label(self, &label.to_native())
    }

    fn run(&mut self) -> PlatingResult<()>
    where
        CHILDREN: ChildOf<Self> + Native<Cocoa>,
    {
        <Self as CocoaApp<STATE, CHILDREN>>::run(self)
    }
}
