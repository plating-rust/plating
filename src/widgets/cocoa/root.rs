/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::defs::CocoaDefaultHandleType;
use crate::widgets::cocoa::error::{CocoaError, CocoaResult};
use crate::widgets::cocoa::CocoaSystem;
use crate::widgets::events::{LifecycleHandler, ListenerType};
use crate::widgets::generic::{NativeRoot, RootHandlerTrait, RootParameters};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Named, OutletHolder, WidgetPointer};
use crate::widgets::{RootChildren, System, Widget};

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSRunningApplication,
};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)] //required in cached version
pub struct CocoaRootParameters {
    //todo
}
impl From<RootParameters> for CocoaRootParameters {
    fn from(_generic: RootParameters) -> Self {
        CocoaRootParameters {}
    }
}

#[derive(Debug)]
pub struct CocoaRoot {
    ///auto generate and add via derive(Widget)
    name: String,

    handle: CocoaDefaultHandleType,

    ///auto generate and add via derive(widgetParent(Window))
    main_outlet: OutletHolder<RootChildren<CocoaSystem>, CocoaRoot, CocoaSystem>,
}

impl NativeRoot<CocoaSystem> for CocoaRoot {
    fn run(&self) -> CocoaResult<()> {
        unsafe {
            self.handle.run();
        };
        Ok(())
    }
}

impl RootHandlerTrait for CocoaRoot {
    fn set_exit_handler(&mut self, _handler: Box<impl FnMut()>) {
        todo!()
    }
    fn add_exit_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }
}

impl Widget<CocoaSystem> for CocoaRoot {
    type PARAMS = CocoaRootParameters;

    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        let app = unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            //todo: try to create children here...

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            app
        };
        let mut new_root = CocoaRoot {
            name,
            handle: app,
            main_outlet: OutletHolder::default(),
        };

        new_root.apply(settings)?;

        Ok(new_root)
    }

    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }

    fn apply<T>(&mut self, settings: T) -> CocoaResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let _settings = settings.into();

        Ok(())
    }
}

impl LifecycleHandler for CocoaRoot {
    fn add_create_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }

    fn add_display_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }

    fn add_destroy_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }

    fn add_apply_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }
}

impl Named for CocoaRoot {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}
// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<RootChildren<CocoaSystem>, CocoaSystem> for CocoaRoot {
    type ErrorType = CocoaError;
    type ParentData = ();

    fn children(&self) -> &[WidgetPointer<RootChildren<CocoaSystem>>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<RootChildren<CocoaSystem>>,
    {
        self.main_outlet.add_child(child, &())
    }
}
