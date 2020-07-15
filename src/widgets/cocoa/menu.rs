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
use crate::widgets::utils::{Child, Connectable, Named, OutletHolder};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(PartialEq)]
pub struct CocoaMenuParameters {
    // generic
    pub title: Option<String>,

    //cocoa specific
    pub autoenables_items: Option<bool>,
    //TODO: font
    pub minimum_width: Option<f32>,
    pub allows_context_menu_plugins: Option<bool>,
    pub shows_state_column: Option<bool>,
    pub user_interface_layout_direction: Option<Direction>,
}
impl From<MenuParameters> for CocoaMenuParameters {
    fn from(generic: MenuParameters) -> Self {
        CocoaMenuParameters {
            title: generic.title,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct CocoaMenu {
    ///auto generate and add via derive(Widget)
    name: String,

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

impl Named for CocoaMenu {
    fn name(&self) -> &str {
        &self.name.as_str()
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
    fn remove_by_name<STR: std::borrow::Borrow<str>>(
        &mut self,
        name: STR,
    ) -> Result<MenuChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_name(name)
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

    fn new_with_name<T>(name: String, settings: T) -> PlatingResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
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
            name,
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
