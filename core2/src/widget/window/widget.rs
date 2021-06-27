/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::cocoa::{CocoaWindow, CocoaWindowWidget};
use crate::native::{Native, NativeWidget, ToNative};
use crate::prelude::WidgetBuilder;
use crate::utils::{
    ChildOf,
    ChildrenList,
    ChildrenOutlet,
    MenuOutlet,
    Outlet,
    OutletHolder,
    Property,
    SettingsList,
};
use crate::widget::cocoa::Cocoa;
use crate::widget::properties::Label;
use crate::widget::{Backend, WidgetAbstractionLevel};
use crate::PlatingResult;

#[derive(Debug, Default)]
pub struct WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    pub children: CHILDREN,
    pub menu:     MENU,
}

impl<CHILDREN, MENU> OutletHolder for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
}

impl<CHILDREN, MENU> Outlet<MenuOutlet> for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    type Children = MENU;

    fn get(&self) -> &Self::Children {
        &self.menu
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.menu
    }
}


impl<CHILDREN, MENU> Outlet<ChildrenOutlet> for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    type Children = CHILDREN;

    fn get(&self) -> &Self::Children {
        &self.children
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}



pub trait WindowWidget<STATE, BACKEND>
where
    Self: Sized,
    BACKEND: Backend,
{
    fn apply<SL>(&mut self, settings: SL) -> PlatingResult<()>
    where
        Self: NativeWidget<STATE, Backend = BACKEND>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<BACKEND>,
        <SL as ToNative<BACKEND>>::Result:
            SettingsList + Native<BACKEND> + Property<STATE, Self, BACKEND>;

    fn set_label(&mut self, label: &Label) -> PlatingResult<()>;
}

impl<STATE, CHILDREN, MENU> WidgetBuilder<STATE, Cocoa> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList + ChildOf<Self> + Native<Cocoa>,
    MENU: ChildrenList + ChildOf<Self> + Native<Cocoa>,
{
    type OutletType = WindowOutlet<CHILDREN, MENU>;

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

impl<STATE, CHILDREN, MENU> WindowWidget<STATE, Cocoa> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    fn apply<SL>(&mut self, settings: SL) -> PlatingResult<()>
    where
        Self: NativeWidget<STATE>,
        SL: SettingsList + WidgetAbstractionLevel + ToNative<<Self as NativeWidget<STATE>>::Backend>,
        <SL as ToNative<<Self as NativeWidget<STATE>>::Backend>>::Result: SettingsList
            + Native<<Self as NativeWidget<STATE>>::Backend>
            + Property<STATE, Self, <Self as NativeWidget<STATE>>::Backend>,
    {
        <Self as NativeWidget<STATE>>::apply(self, &settings.to_native())
    }

    fn set_label(&mut self, label: &Label) -> PlatingResult<()> {
        <Self as CocoaWindow<STATE>>::set_label(self, &label.to_native())
    }
}
