/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::defs::CocoaDefaultHandleType;
use crate::widgets::cocoa::CocoaSystem;
use crate::widgets::outlet::Outlet;
use crate::widgets::root::{NativeRoot, RootChildren, RootHandlerTrait, RootParameters};
use crate::widgets::utils::{Named, OutletHolder};
use crate::widgets::{System, Widget};
use crate::PlatingResult;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSRunningApplication,
};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq, Hash)] //required in cached version
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

impl Default for CocoaRoot {
    fn default() -> Self {
        Self::new(CocoaRootParameters::default()).unwrap()
    }
}

impl PartialEq for CocoaRoot {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaRoot {}

impl NativeRoot<CocoaSystem> for CocoaRoot {
    fn run(&self) -> PlatingResult<()> {
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

    fn new_with_name<T>(name: String, settings: T) -> PlatingResult<Self>
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

    fn apply<T>(&mut self, settings: T) -> PlatingResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let _settings = settings.into();

        Ok(())
    }
}

impl Named for CocoaRoot {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}
// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<RootChildren<CocoaSystem>, CocoaSystem> for CocoaRoot {
    type ParentData = ();

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, RootChildren<CocoaSystem>> {
        self.main_outlet.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, RootChildren<CocoaSystem>> {
        self.main_outlet.iter_mut()
    }

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<RootChildren<CocoaSystem>>,
    {
        self.main_outlet.push_child(child, &())
    }

    fn insert_child<T>(&mut self, index: usize, child: T) -> Result<(), anyhow::Error>
    where
        T: Into<RootChildren<CocoaSystem>>,
    {
        self.main_outlet.insert_child(index, child.into(), &())
    }

    fn capacity(&self) -> usize {
        self.main_outlet.capacity()
    }
    fn reserve(&mut self, additional: usize) {
        self.main_outlet.reserve(additional)
    }
    fn reserve_exact(&mut self, additional: usize) {
        self.main_outlet.reserve_exact(additional)
    }
    fn shrink_to_fit(&mut self) {
        self.main_outlet.shrink_to_fit()
    }
    fn clear(&mut self) {
        self.main_outlet.clear()
    }
    fn len(&self) -> usize {
        self.main_outlet.len()
    }
    fn is_empty(&self) -> bool {
        self.main_outlet.is_empty()
    }
    fn remove_by_index(&mut self, index: usize) -> RootChildren<CocoaSystem> {
        self.main_outlet.remove_by_index(index)
    }
    fn remove_by_name<STR: std::borrow::Borrow<str>>(
        &mut self,
        name: STR,
    ) -> Result<RootChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_name(name)
    }
    fn remove_by_predicate<F: FnMut(&RootChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<RootChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_predicate(f)
    }
}
