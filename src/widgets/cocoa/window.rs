/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::lifecycle::{AttachEvent, AttachTopic};
use crate::events::{ListenerType, PermissionResult, PermissionState};
use crate::features::log;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaRoot, CocoaSystem};
use crate::widgets::outlet::Outlet;
use crate::widgets::root::RootChildren;
use crate::widgets::utils::{Child, Connectable, Named, OutletHolder};
use crate::widgets::window::{
    MainMenuChildren, NativeWindow, WindowChildren, WindowHandlerTrait, WindowParameters,
};
use crate::{
    widgets::{System, Widget},
    PlatingResult,
};

use cocoa::appkit::{
    NSApp, NSApplication, NSBackingStoreBuffered, NSEvent, NSMenu, NSWindow, NSWindowDepth,
    NSWindowStyleMask,
};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use core_graphics::base::CGFloat;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use std::fmt;

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
            window_style: if custom_window_style {
                Some(window_style)
            } else {
                None
            },

            ..Default::default()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct CocoaMainMenuParentData {
    //pub menu_item: CocoaDefaultHandleType,
    pub menu: CocoaDefaultHandleType,
}

pub struct CocoaWindow {
    ///auto generate and add via derive(Widget)
    name: String,

    handle: CocoaDefaultHandleType,

    //todo: move to custom cocoa type
    connected: bool,

    ///auto generate and add via derive(widgetParent(Window))
    //todo: move to custom cocoa type
    main_outlet: OutletHolder<WindowChildren<CocoaSystem>, CocoaWindow, CocoaSystem>,
    //todo: move to custom cocoa type
    menu_outlet: OutletHolder<MainMenuChildren<CocoaSystem>, CocoaWindow, CocoaSystem>,

    //todo: move to custom cocoa type
    menubar: Option<CocoaDefaultHandleType>,
    /*attach_handler: Option<
        Box<
            dyn FnMut(&AttachEvent<CocoaRoot, CocoaSystem>, &dyn Named) -> PermissionResult
            >>  ,

    observable: Subject<AttachEvent<CocoaRoot, CocoaSystem>, anyhow::Error>,*/
}

impl fmt::Debug for CocoaWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CocoaWindow")
            .field("name", &self.name())
            .field("handle", &self.handle)
            .field("main_outlet", &self.main_outlet)
            .field("menu_outlet", &self.menu_outlet)
            //.field("attach_handler", &format!("{:p}", &self.attach_handler))
            .finish()
    }
}

impl fmt::Display for CocoaWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<Window>", self.name)
    }
}

impl CocoaWindow {
    pub(self) fn prepare_insertion(&mut self) {
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
    }
    pub(self) fn create_main_menu_parent_data(&self) -> CocoaMainMenuParentData {
        CocoaMainMenuParentData {
            //menu_item: self.menu_item.unwrap(),
            menu: self.menubar.unwrap(),
        }
    }
}

impl PartialEq for CocoaWindow {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaWindow {}

//todo: remove
extern "C" fn mouse_down(obj: &Object, _: Sel, ev: id) {
    log::warn!("mouse down!");
    unsafe {
        let window_alpha: CGFloat = 0.5;
        let _: () = msg_send![super(obj, class!(NSWindow)), setAlphaValue: window_alpha];
        //let _: () = msg_send![obj, setOpaque:true];
        let _: () = msg_send![super(obj, class!(NSWindow)), mouseDown: ev];

        log::error!("{}", ev.timestamp());
    };

    log::warn!("parent done!");
}

impl Widget<CocoaSystem> for CocoaWindow {
    type PARAMS = CocoaWindowParameters;

    fn new_with_name<T>(name: String, settings: T) -> PlatingResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        //set view controller
        let window = unsafe {
            let superclass = class!(NSWindow);
            let mut decl = ClassDecl::new("MW", superclass).unwrap();

            decl.add_method(
                sel!(mouseDown:),
                mouse_down as extern "C" fn(&Object, Sel, id),
            );

            let view_class = decl.register();
            let id: id = msg_send![view_class, alloc];
            id.initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease()
        };
        /*
        let window = unsafe {
            NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                    NSWindowStyleMask::NSTitledWindowMask,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease()
        };*/

        let mut new_window = CocoaWindow {
            name,
            handle: window,
            main_outlet: OutletHolder::default(),
            menu_outlet: OutletHolder::default(),
            menubar: None,
            connected: false,
        };
        new_window.apply(settings)?;
        unsafe {
            window.makeKeyAndOrderFront_(nil);
        }

        Ok(new_window)
    }

    fn apply<T>(&mut self, settings: T) -> PlatingResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let settings = settings.into();
        log::info!("applying settings: {:?}", settings);
        unsafe {
            if let Some(_rect) = settings.rect {
                todo!()
            }
            if let Some(title) = settings.title {
                let title = NSString::alloc(nil).init_str(&title);
                self.handle.setTitle_(title);
            }
            if let Some(alpha_value) = settings.alpha_value {
                self.handle.setAlphaValue_(alpha_value as CGFloat);
            }
            if let Some(_works_when_modal) = settings.works_when_modal {
                todo!()
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
            if let Some(_has_shadow) = settings.has_shadow {
                todo!()
            }
            if let Some(_autorecalculate_content_border_thickness) =
                settings.autorecalculate_content_border_thickness
            {
                todo!()
            }
            if let Some(_prevents_application_termination_when_modal) =
                settings.prevents_application_termination_when_modal
            {
                todo!()
            }
            if let Some(_can_become_visible_without_login) =
                settings.can_become_visible_without_login
            {
                todo!()
            }
            if let Some(_depth_limit) = settings.depth_limit {
                todo!()
            }
            if let Some(_resize_increments) = settings.resize_increments {
                todo!()
            }
            if let Some(window_style) = settings.window_style {
                self.handle.setStyleMask_(window_style);
            }
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

impl Outlet<MainMenuChildren<CocoaSystem>, CocoaSystem> for CocoaWindow {
    type ParentData = CocoaMainMenuParentData;

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, MainMenuChildren<CocoaSystem>> {
        self.menu_outlet.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, MainMenuChildren<CocoaSystem>> {
        self.menu_outlet.iter_mut()
    }

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<MainMenuChildren<CocoaSystem>>,
    {
        self.prepare_insertion();
        self.menu_outlet
            .push_child(child, &self.create_main_menu_parent_data())
    }
    fn insert_child<T>(&mut self, index: usize, child: T) -> Result<(), anyhow::Error>
    where
        T: Into<MainMenuChildren<CocoaSystem>>,
    {
        self.prepare_insertion();

        self.menu_outlet
            .insert_child(index, child, &self.create_main_menu_parent_data())
    }

    fn capacity(&self) -> usize {
        self.menu_outlet.capacity()
    }
    fn reserve(&mut self, additional: usize) {
        self.menu_outlet.reserve(additional)
    }
    fn reserve_exact(&mut self, additional: usize) {
        self.menu_outlet.reserve_exact(additional)
    }
    fn shrink_to_fit(&mut self) {
        self.menu_outlet.shrink_to_fit()
    }
    fn clear(&mut self) {
        self.menu_outlet.clear()
    }
    fn len(&self) -> usize {
        self.menu_outlet.len()
    }
    fn is_empty(&self) -> bool {
        self.menu_outlet.is_empty()
    }
    fn remove_by_index(&mut self, index: usize) -> MainMenuChildren<CocoaSystem> {
        self.menu_outlet.remove_by_index(index)
    }
    fn remove_by_name<STR: std::borrow::Borrow<str>>(
        &mut self,
        name: STR,
    ) -> Result<MainMenuChildren<CocoaSystem>, anyhow::Error> {
        self.menu_outlet.remove_by_name(name)
    }
    fn remove_by_predicate<F: FnMut(&MainMenuChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<MainMenuChildren<CocoaSystem>, anyhow::Error> {
        self.menu_outlet.remove_by_predicate(f)
    }
}

impl Named for CocoaWindow {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

impl From<CocoaWindow> for RootChildren<CocoaSystem> {
    fn from(window: CocoaWindow) -> Self {
        RootChildren::WINDOW(window)
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<WindowChildren<CocoaSystem>, CocoaSystem> for CocoaWindow {
    type ParentData = ();

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, WindowChildren<CocoaSystem>> {
        self.main_outlet.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, WindowChildren<CocoaSystem>> {
        self.main_outlet.iter_mut()
    }

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<WindowChildren<CocoaSystem>>,
    {
        self.main_outlet.push_child(child, &())
    }

    fn insert_child<T>(&mut self, index: usize, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<WindowChildren<CocoaSystem>>,
    {
        self.main_outlet.insert_child(index, child, &())
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
    fn remove_by_index(&mut self, index: usize) -> WindowChildren<CocoaSystem> {
        self.main_outlet.remove_by_index(index)
    }
    fn remove_by_name<STR: std::borrow::Borrow<str>>(
        &mut self,
        name: STR,
    ) -> Result<WindowChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_name(name)
    }
    fn remove_by_predicate<F: FnMut(&WindowChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<WindowChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_predicate(f)
    }
}

impl Child<CocoaRoot, RootChildren<CocoaSystem>, CocoaSystem> for CocoaWindow {
    fn adding_to_parent(&mut self, _parent: &()) {
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

/*
impl AttachTopic<CocoaRoot, CocoaSystem> for CocoaWindow {

    fn observe(&self, when: ListenerType) -> dyn Observable<Item = AttachEvent<CocoaRoot, CocoaSystem>, Err = anyhow::Error> {
        self.observable.clone()
    }

    fn set_handler(&self, handler: Box<impl FnMut(&crate::actions::lifecycle::AttachEvent<CocoaRoot, CocoaSystem>, &dyn Named) -> crate::events::PermissionResult>) {
        self.observe(ListenerType::Before).subscribe(|x| print("yay"));
    }
}*/

impl WindowHandlerTrait<CocoaSystem> for CocoaWindow {}

impl NativeWindow<CocoaSystem> for CocoaWindow {}

impl Connectable for CocoaWindow {
    fn connecting(&mut self) {
        if self.connected {
            panic!("CocoaWindow already connected")
        }

        self.main_outlet.connecting();
        self.menu_outlet.connecting();

        self.connected = true;
    }

    fn disconnecting(&mut self) {
        if !self.connected {
            panic!("CocoaWindow not yet connected")
        }

        self.main_outlet.disconnecting();
        self.menu_outlet.disconnecting();

        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
