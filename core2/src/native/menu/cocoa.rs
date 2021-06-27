/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

use std::pin::Pin;

use crate::backend::cocoa::appkit::NSMenu;
use crate::backend::cocoa::base::nil;
use crate::backend::cocoa::StrongPtr;
use crate::native::cocoa::CocoaInternal;
use crate::native::data::cocoa::CocoaLabel;
use crate::native::{Native, NativeBuilder, NativeChildOf, NativeWidget};
use crate::utils::{ChildrenList, ChildrenOutlet, Outlet, OutletHolder, Property, SettingsList};
use crate::widget::cocoa::Cocoa;
use crate::widget::MenuOutlet;
use crate::PlatingResult;

#[derive(Debug, Default, Clone, Hash)]
pub struct CocoaMenuOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    children: CHILDREN,
}

impl<CHILDREN> OutletHolder for CocoaMenuOutlet<CHILDREN> where CHILDREN: ChildrenList {}

//TODO: derive
//#derive(Native)
//#[cocoa]
impl<CHILDREN> Native<Cocoa> for CocoaMenuOutlet<CHILDREN> where CHILDREN: ChildrenList {}

//TODO: automatically
impl<CHILDREN> Outlet<ChildrenOutlet> for CocoaMenuOutlet<CHILDREN>
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

//TODO: Cocoa macro
impl<CHILDREN> From<MenuOutlet<CHILDREN>> for CocoaMenuOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    fn from(menu_outlet: MenuOutlet<CHILDREN>) -> Self {
        Self {
            children: menu_outlet.children,
        }
    }
}

#[derive(Debug)]
pub struct CocoaMenuWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    data: Pin<Box<CocoaInternal<STATE, CHILDREN, CocoaMenuOutlet<CHILDREN>>>>,
}

//TODO: macro
impl<STATE, CHILDREN> Native<Cocoa> for CocoaMenuWidget<STATE, CHILDREN> where CHILDREN: ChildrenList
{}

//TODO: generic macro, other functions are not allowed in here!
impl<STATE, CHILDREN> CocoaMenuWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    pub fn state(&self) -> &STATE {
        &self.data.state
    }
}

pub trait CocoaMenu<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
    Self: NativeWidget<STATE, Backend = Cocoa> + Sized,
{
    fn set_label(&mut self, label: &CocoaLabel) -> PlatingResult<()>;
}

impl<STATE, CHILDREN> CocoaMenu<STATE, CHILDREN> for CocoaMenuWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    fn set_label(&mut self, _label: &CocoaLabel) -> PlatingResult<()> {
        //todo:
        Ok(())
    }
}


impl<STATE, CHILDREN> NativeBuilder<STATE> for CocoaMenuWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    type OutletType = CocoaMenuOutlet<CHILDREN>;

    fn new_with_state<SL>(
        state: STATE,
        settings: &SL,
        outlet: Self::OutletType,
    ) -> PlatingResult<Self>
    where
        CHILDREN: NativeChildOf<Self, STATE, Cocoa>,
        SL: SettingsList + Property<STATE, Self, Cocoa> + Native<Cocoa>,
    {
        let menu = unsafe {
            let id = StrongPtr::new(NSMenu::alloc(nil));
            //todo
            //id.initWithTitle_();
            id
        };

        let mut result = CocoaMenuWidget {
            data: CocoaInternal::new(menu, state, outlet),
        };
        result.apply(settings)?;

        result.data.outlet.children.setup(&result);

        Ok(result)
    }
}

impl<STATE, CHILDREN> NativeWidget<STATE> for CocoaMenuWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    type InternalHandle = StrongPtr;
    type Backend = Cocoa;

    fn apply<SL>(&mut self, settings: &SL) -> PlatingResult<()>
    where
        SL: SettingsList + Property<STATE, Self, Cocoa> + Native<Cocoa>,
    {
        settings.provide(self)
    }
}
