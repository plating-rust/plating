/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::{Native, NativeWidget, ToNative};
use crate::utils::{ChildrenList, ChildrenOutlet, Outlet, OutletHolder, Property, SettingsList};
use crate::widget::properties::Label;
use crate::widget::{Backend, WidgetAbstractionLevel};
use crate::PlatingResult;

#[derive(Debug, Default, Clone, Hash)]
pub struct MenuOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    pub children: CHILDREN,
}

impl<CHILDREN> OutletHolder for MenuOutlet<CHILDREN> where CHILDREN: ChildrenList {}

//todo: use makro
impl<CHILDREN> Outlet<ChildrenOutlet> for MenuOutlet<CHILDREN>
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

pub trait MenuWidget<STATE, BACKEND>
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
    //todo: Disabled
    //todo: visible

    //todo: font
    //todo: hotkey

    //todo: minimum width, height
}
