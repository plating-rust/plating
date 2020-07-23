/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaMenu, CocoaMenuParentData, CocoaSystem};
use crate::widgets::menu::MenuChildren;
use crate::widgets::menu_item::{MenuItem, MenuItemHandlerTrait, MenuItemParameters};
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::utils::{Child, Connectable, Identity, Parameters};
use crate::widgets::{System, Widget};
use crate::{OptionalCheckedState, PlatingResult};

use cocoa::appkit::{NSMenu, NSMenuItem, NSWindow};
use cocoa::base::nil;
use cocoa::foundation::{NSAutoreleasePool, NSString};

use std::borrow::Borrow;

pub trait CocoaMenuPlatformParameters {
    fn tag(&self) -> &Option<i32>;

    fn set_tag(&mut self, tag: i32) -> &mut Self;
    fn set_tag_optionally(&mut self, tag: Option<i32>) -> &mut Self;
    fn unset_tag(&mut self) -> &mut Self;

    fn state(&self) -> &Option<OptionalCheckedState>;

    fn set_state(&mut self, state: OptionalCheckedState) -> &mut Self;
    fn set_state_optionally(&mut self, state: Option<OptionalCheckedState>) -> &mut Self;
    fn unset_state(&mut self) -> &mut Self;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct CocoaMenuItemParameters {
    // generic
    label: Option<String>,
    enabled: Option<bool>,
    //todo: pub image: Option<NSImage>,

    //cocoa specific
    //todo: pub attributed_title: Option<NSAttributedString>
    tag: Option<i32>,
    state: Option<OptionalCheckedState>,
}

impl Parameters for CocoaMenuItemParameters {
    fn merge(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        if self.label().is_none() {
            self.set_label_optionally(rhs.label);
        }

        if self.enabled().is_none() {
            self.set_enabled_optionally(rhs.enabled);
        }

        if self.tag().is_none() {
            self.set_tag_optionally(rhs.tag);
        }

        if self.state().is_none() {
            self.set_state_optionally(rhs.state);
        }

        Ok(())
    }
    fn on_top(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        self.set_label_optionally(rhs.label);
        self.set_enabled_optionally(rhs.enabled);
        self.set_tag_optionally(rhs.tag);
        self.set_state_optionally(rhs.state);
        Ok(())
    }
}

impl MenuItemParameters for CocoaMenuItemParameters {
    fn label(&self) -> &Option<String> {
        &self.label
    }
    fn set_label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self {
        if let Some(s) = label {
            self.set_label(s);
        }
        self
    }
    fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    fn enabled(&self) -> &Option<bool> {
        &self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = Some(enabled);
        self
    }
    fn set_enabled_optionally(&mut self, enabled: Option<bool>) -> &mut Self {
        if let Some(e) = enabled {
            self.set_enabled(e);
        }
        self
    }
    fn unset_enabled(&mut self) -> &mut Self {
        self.enabled = None;
        self
    }
}

impl CocoaMenuPlatformParameters for CocoaMenuItemParameters {
    fn tag(&self) -> &Option<i32> {
        &self.tag
    }
    fn set_tag(&mut self, tag: i32) -> &mut Self {
        self.tag = Some(tag);
        self
    }
    fn set_tag_optionally(&mut self, tag: Option<i32>) -> &mut Self {
        if let Some(i) = tag {
            self.set_tag(i);
        }
        self
    }
    fn unset_tag(&mut self) -> &mut Self {
        self.tag = None;
        self
    }
    fn state(&self) -> &Option<OptionalCheckedState> {
        &self.state
    }
    fn set_state(&mut self, state: OptionalCheckedState) -> &mut Self {
        self.state = Some(state);
        self
    }
    fn set_state_optionally(&mut self, state: Option<OptionalCheckedState>) -> &mut Self {
        if let Some(state) = state {
            self.set_state(state);
        }
        self
    }
    fn unset_state(&mut self) -> &mut Self {
        self.state = None;
        self
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

    fn new_with_id<STR, PARAMS>(id: STR, settings: PARAMS) -> PlatingResult<Self>
    where
        STR: Into<String>,
        PARAMS: Borrow<Self::PARAMS>,
    {
        let menu_item = unsafe { NSMenuItem::new(nil).autorelease() };
        let mut new_menu_item = Self {
            id: id.into(),
            handle: menu_item,
            connected: false,
        };
        new_menu_item.apply(settings)?;
        Ok(new_menu_item)
    }

    fn apply<PARAMS>(&mut self, settings: PARAMS) -> PlatingResult<()>
    where
        PARAMS: Borrow<Self::PARAMS>,
    {
        let menu_parameters = settings.borrow();
        log::info!("applying settings: {:?}", menu_parameters);
        unsafe {
            if let Some(label) = menu_parameters.label() {
                let label = NSString::alloc(nil).init_str(&label);
                self.handle.setTitle_(label);
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
        false
    }
}

impl From<CocoaMenuItem> for MenuChildren<CocoaSystem> {
    fn from(menu_item: CocoaMenuItem) -> Self {
        Self::ITEM(menu_item)
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
