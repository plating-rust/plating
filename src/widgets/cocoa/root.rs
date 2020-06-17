/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::defs::CocoaDefaultHandleType;
use crate::widgets::cocoa::error::{CocoaError, CocoaResult};
use crate::widgets::cocoa::CocoaSystem;
use crate::widgets::generic::{RootParameters, RootWidgetTrait};
use crate::widgets::System;
use crate::widgets::{
    ChildrenHolder, NativeWidget, Outlet, OutletAdapter, RootChildren, Widget, WidgetHolder,
};

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
    main_outlet: Outlet<RootChildren<CocoaSystem>, CocoaRoot, CocoaSystem>,
}

impl Widget for CocoaRoot {
    type PARAMS = CocoaRootParameters;
}

impl RootWidgetTrait<CocoaSystem> for CocoaRoot {
    fn run(&self) -> CocoaResult<()> {
        unsafe {
            self.handle.run();
        };
        Ok(())
    }
}

impl NativeWidget<CocoaSystem> for CocoaRoot {
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
            main_outlet: Outlet::default(),
        };

        new_root.apply(settings)?;

        Ok(new_root)
    }

    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }

    fn apply<T>(&mut self, settings: T) -> CocoaResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let _settings = settings.into();

        Ok(())
    }
}

impl WidgetHolder for CocoaRoot {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}
// auto generate impl via derive(widgetParent(A, B    ))
impl OutletAdapter<RootChildren<CocoaSystem>, CocoaSystem> for CocoaRoot {
    type ErrorType = CocoaError;
    type ParentData = ();

    fn children(&self) -> &[ChildrenHolder<RootChildren<CocoaSystem>>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<RootChildren<CocoaSystem>>,
    {
        self.main_outlet.add_child(child, &())
    }
}
