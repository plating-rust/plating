/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

use std::pin::Pin;

use crate::backend::cocoa::appkit::{NSBackingStoreBuffered, NSWindow, NSWindowStyleMask};
use crate::backend::cocoa::base::{nil, NO};
use crate::backend::cocoa::foundation::{NSPoint, NSRect, NSSize};
use crate::backend::cocoa::StrongPtr;
use crate::backend::AsBackend;
use crate::native::cocoa::{CocoaAppWidget, CocoaInternal};
use crate::native::data::cocoa::CocoaLabel;
use crate::native::{Native, NativeBuilder, NativeChildOf, NativeWidget};
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
use crate::widget::WindowOutlet;
use crate::PlatingResult;

#[derive(Debug, Default, Clone, Hash)]
pub struct CocoaWindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    pub children: CHILDREN,
    pub menu:     MENU,
}


impl<CHILDREN, MENU> OutletHolder for CocoaWindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{ }

impl<CHILDREN, MENU> Native<Cocoa> for CocoaWindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{ }

impl<CHILDREN, MENU> From<WindowOutlet<CHILDREN, MENU>> for CocoaWindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    fn from(app: WindowOutlet<CHILDREN, MENU>) -> Self {
        Self {
            children: app.children,
            menu:     app.menu,
        }
    }
}

impl<CHILDREN, MENU> Outlet<ChildrenOutlet> for CocoaWindowOutlet<CHILDREN, MENU>
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

impl<CHILDREN, MENU> Outlet<MenuOutlet> for CocoaWindowOutlet<CHILDREN, MENU>
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


#[derive(Debug)]
pub struct CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    data: Pin<Box<CocoaInternal<STATE, CHILDREN, CocoaWindowOutlet<CHILDREN, MENU>>>>,
}

impl<STATE, CHILDREN, MENU> Native<Cocoa> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    MENU: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
}

impl<STATE, CHILDREN, MENU> CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    MENU: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    pub fn state(&self) -> &STATE {
        &self.data.state
    }
}

pub trait CocoaWindow<STATE>
where
    Self: NativeWidget<STATE, Backend = Cocoa> + Sized,
{
    fn set_label(&mut self, label: &CocoaLabel) -> PlatingResult<()>;
}

impl<STATE, CHILDREN, MENU> CocoaWindow<STATE> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    fn set_label(&mut self, _label: &CocoaLabel) -> PlatingResult<()> {
        //todo:
        Ok(())
    }
}

impl<STATE, CHILDREN, MENU> NativeBuilder<STATE> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    MENU: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    type OutletType = CocoaWindowOutlet<CHILDREN, MENU>;

    fn new_with_state<SL>(
        state: STATE,
        settings: &SL,
        outlet: Self::OutletType,
    ) -> PlatingResult<Self>
    where
        CHILDREN: NativeChildOf<Self, STATE, Cocoa>,
        SL: SettingsList + Property<STATE, Self, Cocoa> + Native<Cocoa>,
    {
        let window = unsafe {
            //TODO: build class building helpers!
            let superclass = objc::class!(NSWindow);
            let decl = objc::declare::ClassDecl::new("MW", superclass)
                .expect("ClassDecl::new to return valid data");

            /*decl.add_ivar::<*mut libc::c_void>("widget");

            decl.add_method(
                sel!(mouseDown:),
                mouse_down as extern "C" fn(&Object, Sel, id),
            );*/

            let view_class = decl.register();
            let id = {
                use objc::*;
                StrongPtr::new(objc::msg_send![view_class, alloc])
            };

            id.initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            );
            id
        };

        let mut result = CocoaWindowWidget {
            data: CocoaInternal::new(window, state, outlet),
        };
        result.apply(settings)?;

        result.data.outlet.children.setup(&result);
        result.data.outlet.menu.setup(&result);

        Ok(result)
    }
}

impl<STATE, CHILDREN, MENU> NativeWidget<STATE> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
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

impl<STATE, CHILDREN, MENU> Default for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    STATE: Default,
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    MENU: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    //Self: NativeBuilder<STATE, ()>,
    CocoaWindowOutlet<CHILDREN, MENU>: Default,
{
    fn default() -> Self {
        Self::new(&(), Default::default()).unwrap()
    }
}

impl<STATE, CHILDREN, MENU> AsBackend<StrongPtr> for CocoaWindowWidget<STATE, CHILDREN, MENU>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    MENU: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    fn as_backend(&self) -> &StrongPtr {
        &self.data.native_handle
    }

    fn as_mut_backend(&mut self) -> &mut StrongPtr {
        unsafe {
            let m = Pin::as_mut(&mut self.data);
            let m = Pin::get_unchecked_mut(m);
            &mut m.native_handle
        }
    }
}

impl<StateP, ChildrenP, StateC, ChildrenC, MenuC> ChildOf<CocoaAppWidget<StateP, ChildrenP>>
    for CocoaWindowWidget<StateC, ChildrenC, MenuC>
where
    ChildrenP: ChildrenList + std::fmt::Debug,
    ChildrenC: ChildrenList + std::fmt::Debug + NativeChildOf<Self, StateC, Cocoa>,
    MenuC: ChildrenList + std::fmt::Debug + NativeChildOf<Self, StateC, Cocoa>,
{
    fn setup(&mut self, _parent: &CocoaAppWidget<StateP, ChildrenP>) {}

    fn connect(&self, _parent: &CocoaAppWidget<StateP, ChildrenP>) {
        unsafe {
            self.data.native_handle.makeKeyAndOrderFront_(nil);
            self.data.native_handle.makeMainWindow();
        }

        self.data.outlet.children.connect(self);
        self.data.outlet.menu.connect(self);
    }

    fn disconnect(&self) {
        //todo: invoke message handlers
        unsafe {
            self.data.native_handle.orderOut_(nil);
        }


        self.data.outlet.children.disconnect();
        self.data.outlet.menu.disconnect();
    }
}
