/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::error::{CocoaError, CocoaResult};
use crate::widgets::cocoa::utils::make_ns_string;
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaSystem, CocoaWindow};
use crate::widgets::events::{LifecycleHandler, ListenerType};
use crate::widgets::menu::{MenuChildren, MenuHandlerTrait, MenuParameters, NativeMenu};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Named, OutletHolder};
use crate::widgets::window::MainMenuChildren;
use crate::widgets::{System, Widget};
use crate::Direction;

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
    type ErrorType = CocoaError;
    type ParentData = CocoaMenuParentData;

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, MenuChildren<CocoaSystem>> {
        self.main_outlet.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, MenuChildren<CocoaSystem>> {
        self.main_outlet.iter_mut()
    }

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<MenuChildren<CocoaSystem>>,
    {
        self.main_outlet
            .push_child(child.into(), &self.create_cocoa_menu_parent_data())
    }

    fn insert_child<T>(&mut self, index: usize, child: T) -> Result<(), Self::ErrorType>
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
    fn as_slice(&self) -> &[MenuChildren<CocoaSystem>] {
        self.main_outlet.as_slice()
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

    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
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
        };
        new_menu.apply(settings)?;
        Ok(new_menu)
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

impl LifecycleHandler for CocoaMenu {
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

impl Child<CocoaMenu, MenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenu {}
impl Child<CocoaWindow, MainMenuChildren<CocoaSystem>, CocoaSystem> for CocoaMenu {
    fn adding_to(
        &self,
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

impl NativeMenu<CocoaSystem> for CocoaMenu {}

impl MenuHandlerTrait for CocoaMenu {}
