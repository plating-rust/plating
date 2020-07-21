/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::defs::CocoaDefaultHandleType;
use crate::widgets::cocoa::CocoaSystem;
use crate::widgets::outlet::Outlet;
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::root::{Root, RootChildren, RootHandlerTrait, RootParameters};
use crate::widgets::utils::{Identity, OutletHolder, Parameters};
use crate::widgets::{System, Widget};
use crate::PlatingResult;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSRunningApplication,
};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

pub trait CocoaRootPlatformParameters {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq, Hash)] //required in cached version
pub struct CocoaRootParameters {}

impl Parameters for CocoaRootParameters {
    fn merge(&mut self, _rhs: Self) -> Result<(), anyhow::Error> {
        Ok(())
    }
    fn on_top(&mut self, _rhs: Self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
impl RootParameters for CocoaRootParameters {}
impl CocoaRootPlatformParameters for CocoaRootParameters {}

#[derive(Debug)]
pub struct CocoaRoot {
    ///auto generate and add via derive(Widget)
    id: String,

    handle: CocoaDefaultHandleType,

    ///auto generate and add via derive(widgetParent(Window))
    main_outlet: OutletHolder<RootChildren<CocoaSystem>, CocoaRoot, CocoaSystem>,
}

impl Default for CocoaRoot {
    fn default() -> Self {
        Self::new(&CocoaRootParameters::default())
            .expect("CocoaRootParameters::default to successfully build CocoaRoot")
    }
}

impl PartialEq for CocoaRoot {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaRoot {}

impl Root<CocoaSystem> for CocoaRoot {
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

    fn new_with_id(id: String, settings: &CocoaRootParameters) -> PlatingResult<Self> {
        let app = unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            //todo: try to create children here...

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            app
        };
        let mut new_root = Self {
            id,
            handle: app,
            main_outlet: OutletHolder::default(),
        };

        new_root.apply(settings)?;

        Ok(new_root)
    }

    fn apply(&mut self, _settings: &CocoaRootParameters) -> PlatingResult<()> {
        Ok(())
    }
}
impl NativeWidget<CocoaSystem> for CocoaRoot {
    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Identity for CocoaRoot {
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}
// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<RootChildren<CocoaSystem>, CocoaSystem> for CocoaRoot {
    type ParentData = ();

    fn iter(&self) -> std::slice::Iter<RootChildren<CocoaSystem>> {
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
    fn remove_by_index(&mut self, index: usize) -> Option<RootChildren<CocoaSystem>> {
        self.main_outlet.remove_by_index(index)
    }
    fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<RootChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_id(id)
    }
    fn remove_by_predicate<F: FnMut(&RootChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<RootChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_predicate(f)
    }
}
