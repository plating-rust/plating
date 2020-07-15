/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::utils::make_ns_string;
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaSystem, CocoaWindow};
use crate::widgets::menu::{Menu, MenuChildren, MenuHandlerTrait, MenuParameters};
use crate::widgets::outlet::Outlet;
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::utils::{Child, Connectable, Identity, OutletHolder};
use crate::widgets::window::MainMenuChildren;
use crate::widgets::{System, Widget};
use crate::{Direction, PlatingResult};

use cocoa::appkit::{NSEventModifierFlags, NSMenu, NSMenuItem, NSWindow};
use cocoa::base::nil;
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::*;

/// Data passed to child in the `adding_to` function.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct CocoaMenuParentData {
    pub item: CocoaDefaultHandleType,
    pub menu: CocoaDefaultHandleType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CocoaMenuParameters {
    // generic
    label: Option<String>,

    //cocoa specific
    auto_enables_items: Option<bool>,
    //TODO: font
    allows_context_menu_plugins: Option<bool>,
    shows_state_column: Option<bool>,
    user_interface_layout_direction: Option<Direction>,
}

impl Parameters for CocoaMenuParameters {
    fn merge(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        if self.label().is_none() {
            self.label = rhs.label;
        }

        Ok(())
    }
    fn on_top(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        self.set_label_optionally(rhs.label);

        Ok(())
    }
}

impl MenuParameters for CocoaMenuParameters {
    fn label(&self) -> &Option<String> {
        &self.label
    }

    fn set_label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }
    fn set_label_optionally(&mut self, title: Option<String>) -> &mut Self {
        if let Some(s) = title {
            self.set_label(s);
        }
        self
    }
    fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }
}

impl CocoaMenuPlatformParameters for CocoaMenuParameters {
    fn auto_enables_items(&self) -> &Option<bool> {
        &self.auto_enables_items
    }
    fn set_auto_enables_items(&mut self, auto_enables_items: bool) -> &mut Self {
        self.auto_enables_items = Some(auto_enables_items);
        self
    }
    fn set_auto_enables_items_optionally(&mut self, auto_enables_items: Option<bool>) -> &mut Self {
        if let Some(b) = auto_enables_items {
            self.set_auto_enables_items(b);
        }
        self
    }
    fn unset_auto_enables_items(&mut self) -> &mut Self {
        self.auto_enables_items = None;
        self
    }
    fn allows_context_menu_plugins(&self) -> &Option<bool> {
        &self.allows_context_menu_plugins
    }
    fn set_allows_context_menu_plugins(&mut self, allows_context_menu_plugins: bool) -> &mut Self {
        self.allows_context_menu_plugins = Some(allows_context_menu_plugins);
        self
    }
    fn set_allows_context_menu_plugins_optionally(
        &mut self,
        allows_context_menu_plugins: Option<bool>,
    ) -> &mut Self {
        if let Some(b) = allows_context_menu_plugins {
            self.set_allows_context_menu_plugins(b);
        }
        self
    }
    fn unset_allows_context_menu_plugins(&mut self) -> &mut Self {
        self.allows_context_menu_plugins = None;
        self
    }
    fn shows_state_column(&self) -> &Option<bool> {
        &self.shows_state_column
    }
    fn set_shows_state_column(&mut self, shows_state_column: bool) -> &mut Self {
        self.shows_state_column = Some(shows_state_column);
        self
    }
    fn set_shows_state_column_optionally(&mut self, shows_state_column: Option<bool>) -> &mut Self {
        if let Some(b) = shows_state_column {
            self.set_shows_state_column(b);
        }
        self
    }
    fn unset_shows_state_column(&mut self) -> &mut Self {
        self.shows_state_column = None;
        self
    }
    fn user_interface_layout_direction(&self) -> &Option<Direction> {
        &self.user_interface_layout_direction
    }
    fn set_user_interface_layout_direction(
        &mut self,
        user_interface_layout_direction: Direction,
    ) -> &mut Self {
        self.user_interface_layout_direction = Some(user_interface_layout_direction);
        self
    }
    fn set_user_interface_layout_direction_optionally(
        &mut self,
        user_interface_layout_direction: Option<Direction>,
    ) -> &mut Self {
        if let Some(dir) = user_interface_layout_direction {
            self.set_user_interface_layout_direction(dir);
        }
        self
    }
    fn unset_user_interface_layout_direction(&mut self) -> &mut Self {
        self.user_interface_layout_direction = None;
        self
    }
}

#[derive(Debug)]
pub struct CocoaMenu {
    ///auto generate and add via derive(Widget)
    id: String,

    handle: CocoaDefaultHandleType,
    ///Stores the MenuItem.
    item: CocoaDefaultHandleType,
    ///auto generate and add via derive(widgetParent(Window))
    main_outlet: OutletHolder<MenuChildren<CocoaSystem>, CocoaMenu, CocoaSystem>,

    connected: bool,
}

impl CocoaMenu {
    pub(self) fn create_cocoa_menu_parent_data(&self) -> CocoaMenuParentData {
        CocoaMenuParentData {
            item: self.item,
            menu: self.handle,
        }
    }
}

impl PartialEq for CocoaMenu {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaMenu {}

impl Identity for CocoaMenu {
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<MenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenu {
    type ParentData = CocoaMenuParentData;

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, MenuChildren<CocoaSystem>> {
        self.main_outlet.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, MenuChildren<CocoaSystem>> {
        self.main_outlet.iter_mut()
    }

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<MenuChildren<CocoaSystem>>,
    {
        self.main_outlet
            .push_child(child.into(), &self.create_cocoa_menu_parent_data())
    }

    fn insert_child<T>(&mut self, index: usize, child: T) -> Result<(), anyhow::Error>
    where
        T: Into<MenuChildren<CocoaSystem>>,
    {
        self.main_outlet
            .insert_child(index, child.into(), &self.create_cocoa_menu_parent_data())
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
    fn remove_by_index(&mut self, index: usize) -> MenuChildren<CocoaSystem> {
        self.main_outlet.remove_by_index(index)
    }
    fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<MenuChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_id(id)
    }
    fn remove_by_predicate<F: FnMut(&MenuChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<MenuChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_predicate(f)
    }
}

impl Widget<CocoaSystem> for CocoaMenu {
    type PARAMS = CocoaMenuParameters;

    fn new_with_id(id: String, settings: &CocoaMenuParameters) -> PlatingResult<Self> {
        log::info!("Creating menu");
        let menu = unsafe {
            let menu = NSMenu::alloc(nil);
            menu.initWithTitle_(make_ns_string("Bearbeiten"));
            menu.autorelease()
        };
        let menu_item = unsafe {
            let item = NSMenuItem::alloc(nil).autorelease();
            item.setSubmenu_(menu);
            item
        };
        let mut new_menu = CocoaMenu {
            id,
            handle: menu,
            item: menu_item,
            main_outlet: OutletHolder::default(),
            connected: false,
        };
        new_menu.apply(settings)?;
        Ok(new_menu)
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

impl NativeWidget<CocoaSystem> for CocoaMenu {
    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Child<CocoaMenu, MenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenu {
    fn adding_to_parent(
        &mut self,
        _parent: &<CocoaMenu as Outlet<MenuChildren<CocoaSystem>, CocoaSystem>>::ParentData,
    ) {
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
impl Child<CocoaWindow, MainMenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenu {
    fn removing_from_parent(&mut self) {
        //todo: invoke message handlers
    }
    fn added(&self) -> bool {
        //todo: get 'parent' value and check if not empty!
        return false;
    }

    fn adding_to_parent(
        &mut self,
        parent: &<CocoaWindow as Outlet<MainMenuChildren<CocoaSystem>, CocoaSystem>>::ParentData,
    ) {
        log::info!("Adding menu to window!");
        unsafe {
            parent.menu.addItem_(self.item);

            let undo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                make_ns_string("sfsadf"),
                sel!(day:),
                make_ns_string("z"),
            );
            self.handle.addItem_(undo);
            self.handle
                .addItem_(NSMenuItem::separatorItem(nil).autorelease());

            let redo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                make_ns_string("Redo"),
                sel!(week:),
                make_ns_string("z"),
            );
            redo.setKeyEquivalentModifierMask_(
                NSEventModifierFlags::NSShiftKeyMask | NSEventModifierFlags::NSCommandKeyMask,
            );
            self.handle.addItem_(redo);
        }
        /*let menubar = parent.menubar.unwrap();
        unsafe {
            menubar.setSubmenu_(self.item)
        }*/
    }
}

impl From<CocoaMenu> for MenuChildren<CocoaSystem> {
    fn from(menu: CocoaMenu) -> Self {
        Self::MENU(menu)
    }
}

impl From<CocoaMenu> for MainMenuChildren<CocoaSystem> {
    fn from(menu: CocoaMenu) -> Self {
        Self::MENU(menu)
    }
}

impl Menu<CocoaSystem> for CocoaMenu {}

impl MenuHandlerTrait for CocoaMenu {}

impl Connectable for CocoaMenu {
    fn connecting(&mut self) {
        if self.connected {
            panic!("CocoaMenu already connected")
        }

        self.main_outlet.connecting();

        self.connected = true;
    }

    fn disconnecting(&mut self) {
        if !self.connected {
            panic!("CocoaMenu not yet connected")
        }

        self.main_outlet.disconnecting();

        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
