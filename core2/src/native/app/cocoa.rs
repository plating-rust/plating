/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

use std::ffi::c_void;
use std::pin::Pin;

use crate::backend::cocoa::appkit::{
    NSApp,
    NSApplication,
    NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular,
    NSRunningApplication,
};
use crate::backend::cocoa::base::{id, nil};
use crate::backend::cocoa::core_foundation::base::{kCFAllocatorDefault, TCFType};
use crate::backend::cocoa::core_foundation::runloop::{
    kCFRunLoopBeforeWaiting,
    CFRunLoopActivity,
    CFRunLoopObserver,
    CFRunLoopObserverContext,
    CFRunLoopObserverCreate,
    CFRunLoopObserverRef,
};
use crate::native::data::cocoa::CocoaLabel;
use crate::native::{Native, NativeBuilder, NativeChildOf, NativeWidget};
use crate::utils::{ChildrenList, ChildrenOutlet, Outlet, OutletHolder, Property, SettingsList};
use crate::widget::cocoa::Cocoa;
use crate::widget::AppOutlet;
use crate::PlatingResult;

#[derive(Debug, Default)]
pub struct CocoaAppOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    children: CHILDREN,
}

impl<CHILDREN> OutletHolder for CocoaAppOutlet<CHILDREN> where CHILDREN: ChildrenList {}

impl<CHILDREN> Native<Cocoa> for CocoaAppOutlet<CHILDREN> where CHILDREN: ChildrenList {}

impl<CHILDREN> From<AppOutlet<CHILDREN>> for CocoaAppOutlet<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    fn from(app: AppOutlet<CHILDREN>) -> Self {
        Self {
            children: app.children,
        }
    }
}

impl<CHILDREN> Outlet<ChildrenOutlet> for CocoaAppOutlet<CHILDREN>
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

#[derive(Debug)]
struct CocoaAppInternal<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    state:  STATE,
    outlet: CocoaAppOutlet<CHILDREN>,

    native_handle: id,

    unpinned: std::marker::PhantomPinned,
}

impl<STATE, CHILDREN> CocoaAppInternal<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    fn tick(&mut self) {
        println!("yay");
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    data: Pin<Box<CocoaAppInternal<STATE, CHILDREN>>>,

    _phantom: std::marker::PhantomData<STATE>,
}

impl<STATE, CHILDREN> CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    //TODO: figure out how we want to access children!
    /*
    fn children(&self) -> &CHILDREN {
        &self.data.outlet.children
    }
    fn children_mut(&mut self) -> &mut CHILDREN {
        unsafe {
            let a = self.data.as_mut();
            let b = a.get_unchecked_mut();
            &mut b.outlet.children
        }
    }*/

    extern "C" fn run_loop_tick(
        _observer: CFRunLoopObserverRef,
        _activity: CFRunLoopActivity,
        info: *mut c_void,
    ) {
        let info: *mut ObserverInfo<STATE, CHILDREN> = unsafe { std::mem::transmute(info) };
        unsafe { (*(*info).app).tick() };
    }
}

impl<STATE, CHILDREN> Native<Cocoa> for CocoaAppWidget<STATE, CHILDREN> where CHILDREN: ChildrenList {}

pub trait CocoaApp<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
    Self: NativeWidget<STATE, Backend = Cocoa> + Sized,
{
    fn set_label(&mut self, label: &CocoaLabel) -> PlatingResult<()>;

    fn run(&mut self) -> PlatingResult<()>
    where
        CHILDREN: NativeChildOf<Self, STATE, Cocoa>;
}

impl<STATE, CHILDREN> CocoaApp<STATE, CHILDREN> for CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    fn set_label(&mut self, _label: &CocoaLabel) -> PlatingResult<()> {
        //todo: self.data.native_handle.set_label(label);
        Ok(())
    }

    fn run(&mut self) -> PlatingResult<()>
    where
        CHILDREN: NativeChildOf<Self, STATE, Cocoa>,
    {
        unsafe {
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            self.data.native_handle.finishLaunching();
        }

        self.data.outlet.children.connect(&*self);

        let app_pointer = unsafe {
            let m = Pin::as_mut(&mut self.data);
            Pin::get_unchecked_mut(m)
        };
        let mut info = ObserverInfo { app: app_pointer };
        let info_ref: *mut ObserverInfo<STATE, CHILDREN> = &mut info;

        let mut context = CFRunLoopObserverContext {
            version: 0,
            info: info_ref as *mut c_void,
            retain: None,
            release: None,
            copyDescription: None,
        };

        unsafe {
            let observer: CFRunLoopObserver =
                TCFType::wrap_under_create_rule(CFRunLoopObserverCreate(
                    kCFAllocatorDefault,
                    kCFRunLoopBeforeWaiting,
                    true as u8,
                    0,
                    Self::run_loop_tick,
                    &mut context,
                ));

            let run_loop = core_foundation::runloop::CFRunLoop::get_main();
            run_loop.add_observer(&observer, core_foundation::runloop::kCFRunLoopDefaultMode);
            self.data.native_handle.run();
        };
        Ok(())
    }
}

impl<STATE, CHILDREN> NativeBuilder<STATE> for CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
{
    type OutletType = CocoaAppOutlet<CHILDREN>;

    fn new_with_state<SL>(
        state: STATE,
        settings: &SL,
        outlet: CocoaAppOutlet<CHILDREN>,
    ) -> PlatingResult<Self>
    where
        CHILDREN: NativeChildOf<Self, STATE, Cocoa>,
        SL: SettingsList + Property<STATE, Self, Cocoa> + Native<Cocoa>,
    {
        let app = unsafe {
            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            app
        };
        let mut result = CocoaAppWidget {
            data:     Box::pin(CocoaAppInternal {
                native_handle: app,
                state,
                outlet,

                unpinned: std::marker::PhantomPinned,
            }),
            _phantom: std::marker::PhantomData,
        };
        result.apply(settings)?;

        result.data.outlet.children.setup(&result);

        Ok(result)
    }
}

impl<STATE, CHILDREN> NativeWidget<STATE> for CocoaAppWidget<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    type InternalHandle = id;
    type Backend = Cocoa;

    fn apply<SL>(&mut self, settings: &SL) -> PlatingResult<()>
    where
        SL: SettingsList + Property<STATE, Self, Cocoa> + Native<Cocoa>,
    {
        settings.provide(self)
    }
}


impl<STATE, CHILDREN> Default for CocoaAppWidget<STATE, CHILDREN>
where
    STATE: Default,
    CHILDREN: ChildrenList + NativeChildOf<Self, STATE, Cocoa>,
    CocoaAppOutlet<CHILDREN>: Default,
{
    fn default() -> Self {
        Self::new(&(), Default::default()).unwrap()
    }
}


struct ObserverInfo<STATE, CHILDREN>
where
    CHILDREN: ChildrenList,
{
    app: *mut CocoaAppInternal<STATE, CHILDREN>,
}
