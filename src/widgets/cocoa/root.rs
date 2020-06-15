/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log::info;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::generic::RootParameters;
use crate::widgets::{
    ChildrenHolder, NativeWidget, Outlet, RootChildren, Widget, WidgetHolder, OutletAdapter,
};
use crate::widgets::cocoa::error::{
    CocoaError, CocoaResult
};
use crate::widgets::cocoa::defs::CocoaDefaultHandleType;

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
    main_outlet: Outlet<RootChildren, CocoaRoot>,
}

impl Widget for CocoaRoot {
    type PARAMS = CocoaRootParameters;
}

impl CocoaRoot {
    pub fn run(&self) -> CocoaResult<()> {
        unsafe {
            self.handle.run();
        };
        Ok(())
    }
}

impl NativeWidget for CocoaRoot {
    type InternalHandle = CocoaDefaultHandleType;
    type ErrorType = CocoaError;

    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        let mut app = unsafe {
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

    fn native(&self) -> &Self::InternalHandle {
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
impl OutletAdapter<RootChildren> for CocoaRoot {
    type AdditionResult = CocoaResult<()>;
    type ParentData = ();

    fn children(&self) -> &[ChildrenHolder<RootChildren>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<RootChildren>,
    {
        self.main_outlet.add_child(child, &())
    }
}
