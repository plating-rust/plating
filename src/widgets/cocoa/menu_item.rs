/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::delegates::CocoaWindowDelegate;
use crate::widgets::cocoa::error::CocoaResult;
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaMenu, CocoaMenuParentData, CocoaSystem};
use crate::widgets::{generic::MenuItemParameters, WidgetType};
use crate::widgets::{Child, MenuChildren, NativeWidget, System, Widget, WidgetHolder};
use crate::CheckedState;

use cocoa::appkit::{NSMenu, NSMenuItem, NSWindow};
use cocoa::base::nil;
use cocoa::foundation::{NSAutoreleasePool, NSString};

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)]
pub struct CocoaMenuItemParameters {
    // generic
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_hidden: Option<bool>,
    //todo: pub image: Option<NSImage>,

    //cocoa specific
    //todo: pub attributed_title: Option<NSAttributedString>
    pub tag: Option<i32>,
    pub state: Option<CheckedState>,
}
impl From<MenuItemParameters> for CocoaMenuItemParameters {
    fn from(generic: MenuItemParameters) -> Self {
        CocoaMenuItemParameters {
            title: generic.title,
            is_enabled: generic.is_enabled,
            is_hidden: generic.is_hidden,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct CocoaMenuItem {
    ///auto generate and add via derive(Widget)
    name: String,

    handle: CocoaDefaultHandleType,
}
impl Widget for CocoaMenuItem {
    type PARAMS = CocoaMenuItemParameters;
}
impl WidgetHolder for CocoaMenuItem {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

impl NativeWidget<CocoaSystem> for CocoaMenuItem {
    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        let menu_item = unsafe { NSMenuItem::new(nil).autorelease() };
        let mut new_menu_item = CocoaMenuItem {
            name,
            handle: menu_item,
        };
        new_menu_item.apply(settings)?;
        Ok(new_menu_item)
    }

    fn apply<T>(&mut self, settings: T) -> CocoaResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let settings = settings.into();
        log::info!("applying settings: {:?}", settings);
        unsafe {
            if let Some(title) = settings.title {
                let title = NSString::alloc(nil).init_str(&title);
                self.handle.setTitle_(title);
            }
            //todo: more
        }

        Ok(())
    }

    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Child<CocoaMenu, MenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenuItem {
    fn adding_to(&self, parent: &CocoaMenuParentData) {
        unsafe {
            parent.menu.addItem_(self.handle);
        }
    }
}

impl From<CocoaMenuItem> for MenuChildren<CocoaSystem> {
    fn from(menu_item: CocoaMenuItem) -> Self {
        MenuChildren::ITEM(WidgetType::NATIVE(menu_item))
    }
}
