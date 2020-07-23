/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log;
use crate::features::serde::Serialize;
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaRoot, CocoaSystem};
use crate::widgets::outlet::Outlet;
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::root::RootChildren;
use crate::widgets::utils::{Child, Connectable, Identity, OutletHolder};
use crate::widgets::window::{
    MainMenuChildren, Window, WindowChildren, WindowHandlerTrait, WindowParameters,
};
use crate::widgets::{System, Widget};
use crate::{prelude::Parameters, PlatingResult};

use cocoa::appkit::{
    NSApp, NSApplication, NSBackingStoreBuffered, NSEvent, NSMenu, NSWindow,
    NSWindowCollectionBehavior, NSWindowStyleMask,
};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use core_graphics::base::CGFloat;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use plating_macros::bitflag_parameter;
use std::borrow::Borrow;
use std::fmt;

bitflag_parameter! {
    pub StyleMaskParameter(NSWindowStyleMask: NSUInteger);
}

bitflag_parameter! {
    pub WindowCollectionBehaviourParameter(NSWindowCollectionBehavior: NSUInteger);
}

pub trait CocoaWindowPlatformParameters {
    fn alpha_value(&self) -> &Option<f32>;

    fn set_alpha_value(&mut self, tag: f32) -> &mut Self;
    fn set_alpha_value_optionally(&mut self, tag: Option<f32>) -> &mut Self;
    fn unset_alpha_value(&mut self) -> &mut Self;

    fn window_style(&self) -> &Option<StyleMaskParameter>;

    fn set_window_style(&mut self, style: StyleMaskParameter) -> &mut Self;
    fn set_window_style_optionally(&mut self, style: Option<StyleMaskParameter>) -> &mut Self;
    fn unset_window_style(&mut self) -> &mut Self;
    //todo: finalize
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
//todo: Deserialize, Serialize
pub struct CocoaWindowParameters {
    //from generic
    label: Option<String>,

    //todo: background_color: Option<Color>,
    alpha_value: Option<f32>,

    works_when_modal: Option<bool>,

    //todo: pub color_space: Option<NSColorSpace>,
    //todo: can_hide: Option<bool>,
    //todo: hides_on_deactivate: Option<bool>,

    //todo: collection_behavior: Option<WindowCollectionBehaviourParameter>,
    //todo: is_opaque: Option<bool>,
    //todo: has_shadow: Option<bool>,

    //todo: autorecalculate_content_border_thickness: Option<bool>,
    //todo: prevents_application_termination_when_modal: Option<bool>,
    //todo: can_become_visible_without_login: Option<bool>,
    //todo: seems to be missing in cocoa:
    //todo: sharing_type: Option<NSWindowSharingType>,
    //todo: depth_limit: Option<NSWindowDepth>,

    //todo: resize_increments: Option<crate::Vec2<f32>>,
    window_style: Option<StyleMaskParameter>,
}

impl WindowParameters for CocoaWindowParameters {
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

    fn resizable(&self) -> Option<bool> {
        match &self.window_style {
            Some(bitfield) => bitfield.is_set(NSWindowStyleMask::NSResizableWindowMask),
            None => None,
        }
    }
    fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        match &mut self.window_style {
            Some(field) => {
                field.set(resizable, NSWindowStyleMask::NSResizableWindowMask);
                self
            }
            None => {
                self.window_style = Some(StyleMaskParameter::default());
                self.set_resizable(resizable)
            }
        }
    }
    fn set_resizable_optionally(&mut self, resizable: Option<bool>) -> &mut Self {
        if let Some(b) = resizable {
            self.set_resizable(b);
        }
        self
    }
    fn unset_resizable(&mut self) -> &mut Self {
        if let Some(bitfield) = &mut self.window_style {
            bitfield.unset(NSWindowStyleMask::NSResizableWindowMask)
        }
        self
    }
    fn closable(&self) -> Option<bool> {
        match &self.window_style {
            Some(bitfield) => bitfield.is_set(NSWindowStyleMask::NSClosableWindowMask),
            None => None,
        }
    }
    fn set_closable(&mut self, closable: bool) -> &mut Self {
        match &mut self.window_style {
            Some(field) => {
                field.set(closable, NSWindowStyleMask::NSClosableWindowMask);
                self
            }
            None => {
                self.window_style = Some(StyleMaskParameter::default());
                self.set_closable(closable)
            }
        }
    }
    fn set_closable_optionally(&mut self, closable: Option<bool>) -> &mut Self {
        if let Some(b) = closable {
            self.set_closable(b);
        }
        self
    }
    fn unset_closable(&mut self) -> &mut Self {
        if let Some(bitfield) = &mut self.window_style {
            bitfield.unset(NSWindowStyleMask::NSClosableWindowMask)
        }
        self
    }
    fn miniaturizable(&self) -> Option<bool> {
        match &self.window_style {
            Some(bitfield) => bitfield.is_set(NSWindowStyleMask::NSMiniaturizableWindowMask),
            None => None,
        }
    }
    fn set_miniaturizable(&mut self, miniaturizable: bool) -> &mut Self {
        match &mut self.window_style {
            Some(field) => {
                field.set(
                    miniaturizable,
                    NSWindowStyleMask::NSMiniaturizableWindowMask,
                );
                self
            }
            None => {
                self.window_style = Some(StyleMaskParameter::default());
                self.set_miniaturizable(miniaturizable)
            }
        }
    }
    fn set_miniaturizable_optionally(&mut self, miniaturizable: Option<bool>) -> &mut Self {
        if let Some(b) = miniaturizable {
            self.set_miniaturizable(b);
        }
        self
    }
    fn unset_miniaturizable(&mut self) -> &mut Self {
        if let Some(bitfield) = &mut self.window_style {
            bitfield.unset(NSWindowStyleMask::NSMiniaturizableWindowMask)
        }
        self
    }
    fn maximizable(&self) -> Option<bool> {
        match &self.window_style {
            Some(bitfield) => bitfield.is_set(NSWindowStyleMask::NSFullScreenWindowMask),
            None => None,
        }
    }
    fn set_maximizable(&mut self, maximizable: bool) -> &mut Self {
        match &mut self.window_style {
            Some(field) => {
                field.set(maximizable, NSWindowStyleMask::NSFullScreenWindowMask);
                self
            }
            None => {
                self.window_style = Some(StyleMaskParameter::default());
                self.set_maximizable(maximizable)
            }
        }
    }
    fn set_maximizable_optionally(&mut self, maximizable: Option<bool>) -> &mut Self {
        if let Some(b) = maximizable {
            self.set_maximizable(b);
        }
        self
    }

    fn unset_maximizable(&mut self) -> &mut Self {
        if let Some(bitfield) = &mut self.window_style {
            bitfield.unset(NSWindowStyleMask::NSFullScreenWindowMask)
        }
        self
    }
    fn fullscreenable(&self) -> Option<bool> {
        match &self.window_style {
            Some(bitfield) => bitfield.is_set(NSWindowStyleMask::NSFullSizeContentViewWindowMask),
            None => None,
        }
    }
    fn set_fullscreenable(&mut self, fullscreenable: bool) -> &mut Self {
        match &mut self.window_style {
            Some(field) => {
                field.set(
                    fullscreenable,
                    NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                );
                self
            }
            None => {
                self.window_style = Some(StyleMaskParameter::default());
                self.set_fullscreenable(fullscreenable)
            }
        }
    }
    fn set_fullscreenable_optionally(&mut self, fullscreenable: Option<bool>) -> &mut Self {
        if let Some(b) = fullscreenable {
            self.set_fullscreenable(b);
        }
        self
    }
    fn unset_fullscreenable(&mut self) -> &mut Self {
        if let Some(bitfield) = &mut self.window_style {
            bitfield.unset(NSWindowStyleMask::NSFullSizeContentViewWindowMask)
        }
        self
    }
}

impl Parameters for CocoaWindowParameters {
    fn merge(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        if self.label().is_none() {
            self.set_label_optionally(rhs.label);
        }

        match (&mut self.window_style(), rhs.window_style) {
            (_, None) => { /* nothing todo */ }
            (None, Some(rh_style)) => {
                self.set_window_style(rh_style);
            }
            (Some(lh_style), Some(rh_style)) => {
                let new_value = lh_style.merge_unchecked(rh_style);
                self.set_window_style(new_value);
            }
        }

        //todo: merge cocoa specific stuff
        Ok(())
    }
    fn on_top(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        self.set_label_optionally(rhs.label);

        match (self.window_style(), rhs.window_style) {
            (_, None) => { /* nothing todo */ }
            (_, Some(rh_style)) => {
                self.set_window_style(rh_style);
            }
        }

        Ok(())
    }
}

impl CocoaWindowPlatformParameters for CocoaWindowParameters {
    fn alpha_value(&self) -> &Option<f32> {
        &self.alpha_value
    }
    fn set_alpha_value(&mut self, alpha: f32) -> &mut Self {
        self.alpha_value = Some(alpha);
        self
    }
    fn set_alpha_value_optionally(&mut self, alpha: Option<f32>) -> &mut Self {
        if let Some(f) = alpha {
            self.set_alpha_value(f);
        }
        self
    }
    fn unset_alpha_value(&mut self) -> &mut Self {
        self.alpha_value = None;
        self
    }
    fn window_style(&self) -> &Option<StyleMaskParameter> {
        &self.window_style
    }
    fn set_window_style(&mut self, style: StyleMaskParameter) -> &mut Self {
        self.window_style = Some(style);
        self
    }
    fn set_window_style_optionally(&mut self, style: Option<StyleMaskParameter>) -> &mut Self {
        if let Some(s) = style {
            self.set_window_style(s);
        }
        self
    }
    fn unset_window_style(&mut self) -> &mut Self {
        self.window_style = None;
        self
    }
}

/*
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
}*/

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct CocoaMainMenuParentData {
    //pub menu_item: CocoaDefaultHandleType,
    pub menu: CocoaDefaultHandleType,
}

pub struct CocoaWindow {
    ///auto generate and add via derive(Widget)
    id: String,

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

impl Default for CocoaWindow {
    fn default() -> Self {
        Self::new(&CocoaWindowParameters::default())
            .expect("CocoaWindowParameters::default should be valid to build CocoaWindow")
    }
}

impl fmt::Debug for CocoaWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CocoaWindow")
            .field("name", &self.id())
            .field("handle", &self.handle)
            .field("main_outlet", &self.main_outlet)
            .field("menu_outlet", &self.menu_outlet)
            //.field("attach_handler", &format!("{:p}", &self.attach_handler))
            .finish()
    }
}

impl fmt::Display for CocoaWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<Window>", self.id())
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
            menu: self.menubar.expect("menubar valid at this point."),
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

    fn new_with_id<STR, PARAMS>(id: STR, settings: PARAMS) -> PlatingResult<Self>
    where
        STR: Into<String>,
        PARAMS: Borrow<Self::PARAMS>,
    {
        //set view controller
        let window = unsafe {
            let superclass = class!(NSWindow);
            let mut decl =
                ClassDecl::new("MW", superclass).expect("ClassDecl::new to return valid data");

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

        let mut new_window = Self {
            id: id.into(),
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

    fn apply<PARAMS>(&mut self, settings: PARAMS) -> PlatingResult<()>
    where
        PARAMS: Borrow<Self::PARAMS>,
    {
        let window_parameters = settings.borrow();
        log::info!("applying settings: {:?}", window_parameters);

        unsafe {
            if let Some(label) = &window_parameters.label {
                let title = NSString::alloc(nil).init_str(label);
                self.handle.setTitle_(title);
            }
            if let Some(alpha_value) = window_parameters.alpha_value {
                self.handle.setAlphaValue_(alpha_value.into());
            }
            /*todo:
            if let Some(can_hide) = window_parameters.can_hide {
                self.handle.setCanHide_(can_hide as i8);
            }
            if let Some(hides_on_deactivate) = window_parameters.hides_on_deactivate {
                self.handle.setHidesOnDeactivate_(hides_on_deactivate as i8);
            }
            if let Some(is_opaque) = window_parameters.is_opaque {
                self.handle.setOpaque_(is_opaque as i8);
            }*/
            if let Some(window_style) = window_parameters.window_style {
                let old_mask = self.handle.styleMask();
                self.handle.setStyleMask_(
                    window_style
                        .apply_into(old_mask)
                        .expect("Cleanly applying window style mask."),
                );
            }
        }

        Ok(())
    }
}

impl NativeWidget<CocoaSystem> for CocoaWindow {
    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Outlet<MainMenuChildren<CocoaSystem>, CocoaSystem> for CocoaWindow {
    type ParentData = CocoaMainMenuParentData;

    fn iter(&self) -> std::slice::Iter<MainMenuChildren<CocoaSystem>> {
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
    fn remove_by_index(&mut self, index: usize) -> Option<MainMenuChildren<CocoaSystem>> {
        self.menu_outlet.remove_by_index(index)
    }
    fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<MainMenuChildren<CocoaSystem>, anyhow::Error> {
        self.menu_outlet.remove_by_id(id)
    }
    fn remove_by_predicate<F: FnMut(&MainMenuChildren<CocoaSystem>) -> bool>(
        &mut self,
        f: F,
    ) -> Result<MainMenuChildren<CocoaSystem>, anyhow::Error> {
        self.menu_outlet.remove_by_predicate(f)
    }
}

impl Identity for CocoaWindow {
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}

impl From<CocoaWindow> for RootChildren<CocoaSystem> {
    fn from(window: CocoaWindow) -> Self {
        Self::WINDOW(window)
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
impl Outlet<WindowChildren<CocoaSystem>, CocoaSystem> for CocoaWindow {
    type ParentData = ();

    fn iter(&self) -> std::slice::Iter<WindowChildren<CocoaSystem>> {
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
    fn remove_by_index(&mut self, index: usize) -> Option<WindowChildren<CocoaSystem>> {
        self.main_outlet.remove_by_index(index)
    }
    fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<WindowChildren<CocoaSystem>, anyhow::Error> {
        self.main_outlet.remove_by_id(id)
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
        false
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

impl Window<CocoaSystem> for CocoaWindow {}

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
