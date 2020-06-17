/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::Direction;
use crate::features::log;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{MainMenuChildren, MenuChildren, Child, ChildrenHolder, NativeWidget, Outlet, Widget, OutletAdapter, WidgetHolder};
use crate::widgets::generic::{MenuParameters};
use crate::widgets::cocoa::{CocoaWindow, CocoaRoot, CocoaDefaultHandleType};
use crate::widgets::cocoa::error::{CocoaError, CocoaResult};
use crate::widgets::cocoa::utils::make_ns_string;


use objc::*;
use cocoa::base::{selector, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use cocoa::appkit::{NSMenu, NSWindow, NSMenuItem, NSEventModifierFlags};

///Data passed to child when they are added
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
    main_outlet: Outlet<MenuChildren, CocoaMenu>,
}
impl Widget for CocoaMenu {
    type PARAMS = CocoaMenuParameters;
}
impl WidgetHolder for CocoaMenu {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
impl OutletAdapter<MenuChildren> for CocoaMenu {
    type AdditionResult = CocoaResult<()>;
    type ParentData = CocoaMenuParentData;

    fn children(&self) -> &[ChildrenHolder<MenuChildren>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<MenuChildren>,
    {
        self.main_outlet.add_child(
            child.into(), 
            &CocoaMenuParentData {
                item: self.item,
                menu: self.handle,
            }
        )
    }
}

impl NativeWidget for CocoaMenu {
    type InternalHandle = CocoaDefaultHandleType;
    type ErrorType = CocoaError;

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
            main_outlet: Outlet::default(),
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

    fn native(&self) -> &Self::InternalHandle {
        &self.handle
    }
}

impl Child<CocoaMenu, MenuChildren> for CocoaMenu {}
impl Child<CocoaWindow, MainMenuChildren> for CocoaMenu {
    fn adding_to(&self, parent: &<CocoaWindow as OutletAdapter<MainMenuChildren>>::ParentData) {
        log::info!("Adding menu to window!");
        unsafe {
            parent.menu.addItem_(self.item);
        
        
            
            let undo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                make_ns_string("sfsadf"),
                sel!(day:),
                make_ns_string("z"),
            );
            self.handle.addItem_(undo);
            self.handle.addItem_(NSMenuItem::separatorItem(nil).autorelease());
            
            let redo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                make_ns_string("Redo"),
                sel!(week:),
                make_ns_string("z"),
            );
            redo.setKeyEquivalentModifierMask_(
                NSEventModifierFlags::NSShiftKeyMask | 
                NSEventModifierFlags::NSCommandKeyMask
            );
            self.handle.addItem_(redo);
        }
        /*let menubar = parent.menubar.unwrap();
        unsafe {
            menubar.setSubmenu_(self.item)
        }*/

    }
}