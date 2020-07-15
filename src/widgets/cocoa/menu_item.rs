/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaMenu, CocoaMenuParentData, CocoaSystem};
use crate::widgets::menu::MenuChildren;
use crate::widgets::menu_item::{MenuItem, MenuItemHandlerTrait, MenuItemParameters};
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::utils::{Child, Connectable, Identity};
use crate::widgets::{System, Widget};
use crate::{CheckedState, PlatingResult};

use cocoa::appkit::{NSMenu, NSMenuItem, NSWindow};
use cocoa::base::nil;
use cocoa::foundation::{NSAutoreleasePool, NSString};

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq, Hash)]
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
    id: String,

    handle: CocoaDefaultHandleType,

    connected: bool,
}

impl PartialEq for CocoaMenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaMenuItem {}

impl MenuItem<CocoaSystem> for CocoaMenuItem {}

impl MenuItemHandlerTrait for CocoaMenuItem {}

impl Identity for CocoaMenuItem {
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}

impl Widget<CocoaSystem> for CocoaMenuItem {
    type PARAMS = CocoaMenuItemParameters;

    fn new_with_id(id: String, settings: &CocoaMenuItemParameters) -> PlatingResult<Self> {
        let menu_item = unsafe { NSMenuItem::new(nil).autorelease() };
        let mut new_menu_item = CocoaMenuItem {
            id,
            handle: menu_item,
            connected: false,
        };
        new_menu_item.apply(settings)?;
        Ok(new_menu_item)
    }

    fn apply<T>(&mut self, settings: T) -> PlatingResult<()>
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
}
impl NativeWidget<CocoaSystem> for CocoaMenuItem {
    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Child<CocoaMenu, MenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenuItem {
    fn adding_to_parent(&mut self, parent: &CocoaMenuParentData) {
        unsafe {
            parent.menu.addItem_(self.handle);
        }

        //todo: invoke message handlers
    }

    fn removing_from_parent(&mut self) {
        //todo: invoke message handlers
    }
    fn added(&self) -> bool {
        //todo: get 'parent' value and check if not empty!
        return false;
    }
}

impl From<CocoaMenuItem> for MenuChildren<CocoaSystem> {
    fn from(menu_item: CocoaMenuItem) -> Self {
        MenuChildren::ITEM(menu_item)
    }
}

impl Connectable for CocoaMenuItem {
    //todo: move
    fn connecting(&mut self) {
        if self.connected {
            panic!("CocoaMenuItem already connected")
        }

        self.connected = true;
    }

    fn disconnecting(&mut self) {
        if !self.connected {
            panic!("CocoaMenuItem not yet connected")
        }

        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
