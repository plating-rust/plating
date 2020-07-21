/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::events::ListenerType;
use crate::features::log::info;

use crate::widgets::button::{Button, ButtonChildren, ButtonHandlerTrait, ButtonParameters};
use crate::widgets::cocoa::{
    CocoaButtonParameters, CocoaDefaultHandleType, CocoaSystem, CocoaWindow,
};
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::utils::{Child, Connectable, Identity};
use crate::widgets::window::WindowChildren;
use crate::widgets::{System, Widget};
use crate::PlatingResult;

#[derive(Debug)]
pub struct CocoaButton {
    ///auto generate and add via derive(widgetParent(A, B    ))
    //main_outlet: Outlet<ButtonChildren, CocoaButton>,
    handle: CocoaDefaultHandleType,

    ///auto generate and add via derive(Widget)
    id: String,

    //todo: move to backend
    connected: bool,
}
/*
impl AttachTopic<CocoaButton> for CocoaButton {
    fn add_listener(when: ListenerType, handler: Box<impl FnMut(&AttachEvent, &dyn Named, &PermissionState)>) {
        todo!{}
    }
    fn set_handler(handler: Box<impl FnMut(&AttachEvent, &dyn Named) -> PermissionResult>) {
        todo!{}
    }
}*/

impl PartialEq for CocoaButton {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl Eq for CocoaButton {}

impl Child<CocoaWindow, WindowChildren<CocoaSystem>, CocoaSystem> for CocoaButton {
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

impl From<CocoaButton> for ButtonChildren<CocoaSystem> {
    fn from(button: CocoaButton) -> Self {
        ButtonChildren::BUTTON(button)
    }
}

impl From<CocoaButton> for WindowChildren<CocoaSystem> {
    fn from(button: CocoaButton) -> Self {
        WindowChildren::BUTTON(button)
    }
}

impl Widget<CocoaSystem> for CocoaButton {
    type PARAMS = CocoaButtonParameters;

    fn new_with_id(id: String, settings: &CocoaButtonParameters) -> PlatingResult<Self> {
        let mut button = CocoaButton {
            id,
            handle: 0 as CocoaDefaultHandleType,
            connected: false,
            //main_outlet: Outlet::<ButtonChildren>::default(),
        };
        button.apply(settings)?;
        Ok(button)
    }

    fn apply(&mut self, settings: &CocoaButtonParameters) -> PlatingResult<()> {
        if settings.label().is_some() {
            info!("settings label");
        }
        Ok(())
    }
}

impl NativeWidget<CocoaSystem> for CocoaButton {
    fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
        &self.handle
    }
    unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
        &mut self.handle
    }
}

impl Identity for CocoaButton {
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}

impl Button<CocoaSystem> for CocoaButton {}

impl ButtonHandlerTrait<CocoaSystem> for CocoaButton {
    fn set_exit_handler(&mut self, _handler: Box<impl FnMut()>) {
        todo!()
    }
    fn add_exit_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }
}

impl Connectable for CocoaButton {
    fn connecting(&mut self) {
        if self.connected {
            panic!("CocoaButton already connected")
        }

        self.connected = true;
    }

    fn disconnecting(&mut self) {
        if !self.connected {
            panic!("CocoaButton not yet connected")
        }

        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}

// auto generate impl via derive(widgetParent(A, B    ))
/*
impl OutletAdapter<ButtonChildren> for CocoaButton {
    type AdditionResult = CocoaResult<()>;

    fn children(&self) -> &[ChildrenHolder<ButtonChildren>] {
        self.main_outlet.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<ButtonChildren>,
    {
        self.main_outlet.add_child(child)
    }
}*/
