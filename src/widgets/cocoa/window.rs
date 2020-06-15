/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */


use crate::features::log;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::generic::WindowParameters;
use crate::widgets::{RootChildren, WindowChildren};
use crate::widgets::{MainMenuChildren, MenuChildren, Child, ChildrenHolder, NativeWidget, Outlet, Widget, WidgetHolder, OutletAdapter};
use crate::widgets::cocoa::{CocoaRoot, CocoaDefaultHandleType};
use crate::widgets::cocoa::error::{CocoaError, CocoaResult};

use cocoa::base::{selector, nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize,
    NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSWindowStyleMask, NSApp, NSWindowCollectionBehavior,
 NSWindowDepth,
NSApplication, NSApplicationActivationPolicyRegular,
NSWindow, NSBackingStoreBuffered, NSColorSpace,
NSMenu, NSMenuItem, NSRunningApplication,
NSApplicationActivateIgnoringOtherApps};
use core_graphics::base::CGFloat;

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(PartialEq)] //required in cached version
pub struct CocoaWindowParameters {
    //from generic
    pub rect: Option<crate::Rect>,

    pub title: Option<String>,

    //TODO: colors! pub backgroundColor: Option<Color>
    pub alpha_value: Option<f32>,
    
    pub works_when_modal: Option<bool>,

    //todo: pub color_space: Option<NSColorSpace>,

    pub can_hide: Option<bool>,
    pub hides_on_deactivate: Option<bool>,

    //todo: pub collection_behavior: Option<NSWindowCollectionBehavior>,

    pub is_opaque: Option<bool>,
    pub has_shadow: Option<bool>,


    pub autorecalculate_content_border_thickness: Option<bool>,
    pub prevents_application_termination_when_modal: Option<bool>,
    pub can_become_visible_without_login: Option<bool>,
    //todo: seems to be missing in cocoa: 
    //pub sharing_type: Option<NSWindowSharingType>,

    pub depth_limit: Option<NSWindowDepth>,

    pub resize_increments: Option<crate::Vec2<f32>>,

    #[serde(default)]
    #[serde(serialize_with = "super::utils::serde::serialize_ns_window_style_mask")]
    #[serde(deserialize_with = "super::utils::serde::deserialize_ns_window_style_mask")]
    pub window_style: Option<NSWindowStyleMask>,
}

impl From<WindowParameters> for CocoaWindowParameters {
    fn from(generic: WindowParameters) -> Self {
        let mut window_style = NSWindowStyleMask::NSTitledWindowMask;
        let mut custom_window_style = false;
        
        if let Some(b) = generic.resizable {
            if b {
                custom_window_style = true;
                window_style |= NSWindowStyleMask::NSResizableWindowMask;
            }
        }
        if let Some(b) = generic.closable {
            if b {
                custom_window_style = true;
                window_style |= NSWindowStyleMask::NSClosableWindowMask;
            }
        }
        if let Some(b) = generic.miniaturizable {
            if b {
                custom_window_style = true;
                window_style |= NSWindowStyleMask::NSMiniaturizableWindowMask;
            }
        }
        if let Some(b) = generic.resizable {
            if b {
                custom_window_style = true;
                window_style |= NSWindowStyleMask::NSResizableWindowMask;
            }
        }
        if let Some(b) = generic.fullscreenable {
            if b {
                custom_window_style = true;
                window_style |= NSWindowStyleMask::NSFullScreenWindowMask;
            }
        }

        if let Some(b) = generic.maximizable {
            if b {
                log::warn!("Maximizable not available in osx");
            }
        }

        CocoaWindowParameters {
            rect: generic.rect,
            title: generic.title,
            window_style: if custom_window_style { Some(window_style) } else { None },


            ..Default::default()
        }
    }
}
/*
#[derive(Debug)]
pub struct MainMenuOutlet {
    window: CocoaWindow,
    
}
impl MainMenuOutlet {
    fn new(window_handle: CocoaDefaultHandleType) -> Self {
        MainMenuOutlet {
            menubar: None,
            outlet: Default::default(),
        }

    }
}*/
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct CocoaMainMenuParentData {
   //pub menu_item: CocoaDefaultHandleType,
   pub menu: CocoaDefaultHandleType,
}

impl OutletAdapter<MainMenuChildren> for CocoaWindow {
    type AdditionResult = CocoaResult<()>;
    type ParentData = CocoaMainMenuParentData;

    fn children(&self) -> &[ChildrenHolder<MainMenuChildren>] {
        &self.menu_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<MainMenuChildren>,
    {
        if self.menubar.is_none() {
            log::info!("Initialize main menu");
            unsafe {
                let menubar = NSMenu::new(nil).autorelease();
                self.menubar = Some(menubar);
                NSApp().setMainMenu_(menubar);

                //let menu_item = NSMenuItem::new(nil).autorelease();
                //menubar.addItem_(menu_item);
                //self.menu_item = Some(menu_item);
            }
        }
        self.menu_outlet.add_child(child, &CocoaMainMenuParentData {
            //menu_item: self.menu_item.unwrap(),
            menu: self.menubar.unwrap(),
        }) //big todo:
    }
}

#[derive(Debug)]
pub struct CocoaWindow {
    ///auto generate and add via derive(Widget)
    name: String,

    handle: CocoaDefaultHandleType,

    ///auto generate and add via derive(widgetParent(Window))
    main_outlet: Outlet<WindowChildren, CocoaWindow>,

    menu_outlet: Outlet<MainMenuChildren, CocoaWindow>,
    
    menubar: Option<CocoaDefaultHandleType>,
    menu_item: Option<CocoaDefaultHandleType>,

}

impl Widget for CocoaWindow {
    type PARAMS = CocoaWindowParameters;
}

impl NativeWidget for CocoaWindow {
    type InternalHandle = CocoaDefaultHandleType;
    type ErrorType = CocoaError;

    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        let window = unsafe {
            NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO
            ).autorelease() 
        };
        
        let mut new_window = CocoaWindow {
            name,
            handle: window,
            main_outlet: Outlet::default(),
            menu_outlet: Outlet::default(),
            menubar: None,
            menu_item: None,
        };
        new_window.apply(settings)?;
        unsafe {
            window.makeKeyAndOrderFront_(nil);
        }
        Ok(new_window)
    }

    fn apply<T>(&mut self, settings: T) -> CocoaResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let settings = settings.into();
        log::info!("applying settings: {:?}", settings);
            unsafe {
            if let Some(rect) = settings.rect {
                todo!()
            }
            if let Some(title) = settings.title {
                let title = NSString::alloc(nil).init_str(&title);
                self.handle.setTitle_(title);
            }
            if let Some(alpha_value) = settings.alpha_value {
                self.handle.setAlphaValue_(alpha_value as CGFloat);
            }
            if let Some(works_when_modal) = settings.works_when_modal {
                
            }
            if let Some(can_hide) = settings.can_hide {
                self.handle.setCanHide_(can_hide as i8);
            }
            if let Some(hides_on_deactivate) = settings.hides_on_deactivate {
                self.handle.setHidesOnDeactivate_(hides_on_deactivate as i8);
            }
            if let Some(is_opaque) = settings.is_opaque {
                self.handle.setOpaque_(is_opaque as i8);
            }
            if let Some(has_shadow) = settings.has_shadow {
                todo!()
            }
            if let Some(autorecalculate_content_border_thickness) = settings.autorecalculate_content_border_thickness {
                todo!()
            }
            if let Some(prevents_application_termination_when_modal) = settings.prevents_application_termination_when_modal {
                todo!()
            }
            if let Some(can_become_visible_without_login) = settings.can_become_visible_without_login {
                todo!()
            }
            if let Some(depth_limit) = settings.depth_limit {
                todo!()
            }
            if let Some(resize_increments) = settings.resize_increments {
                todo!()
            }
            if let Some(window_style) = settings.window_style {
                self.handle.setStyleMask_(window_style);
            }
        }

        Ok(())
    }

    fn native(&self) -> &Self::InternalHandle {
        &self.handle
    }
}

impl WidgetHolder for CocoaWindow {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
impl OutletAdapter<WindowChildren> for CocoaWindow {
    type AdditionResult = CocoaResult<()>;
    type ParentData = ();

    fn children(&self) -> &[ChildrenHolder<WindowChildren>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<WindowChildren>,
    {
        self.main_outlet.add_child(child, &())
    }
}

impl Child<CocoaRoot, RootChildren> for CocoaWindow {}
