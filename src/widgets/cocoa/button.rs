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
use cocoa::base::nil;
use plating_macros::{Identifiable, NativeWidget};

use std::borrow::Borrow;

#[derive(Debug, Eq, Identifiable, NativeWidget)]
#[system = "CocoaSystem"]
pub struct CocoaButton {
    ///auto generate and add via derive(widgetParent(A, B    ))
    //main_outlet: Outlet<ButtonChildren, CocoaButton>,

    #[native_handle]
    handle: CocoaDefaultHandleType,

    #[id]
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
        Self::BUTTON(button)
    }
}

impl From<CocoaButton> for WindowChildren<CocoaSystem> {
    fn from(button: CocoaButton) -> Self {
        Self::BUTTON(button)
    }
}

impl Widget<CocoaSystem> for CocoaButton {
    type PARAMS = CocoaButtonParameters;

    fn new_with_id<STR, PARAMS>(id: STR, settings: PARAMS) -> PlatingResult<Self>
    where
        STR: Into<String>,
        PARAMS: Borrow<CocoaButtonParameters>,
    {
        let mut button = Self {
            id: id.into(),
            handle: nil,
            connected: false,
            //main_outlet: Outlet::<ButtonChildren>::default(),
        };
        button.apply(settings)?;
        Ok(button)
    }

    fn apply<PARAMS>(&mut self, settings: PARAMS) -> PlatingResult<()>
    where
        PARAMS: Borrow<CocoaButtonParameters>,
    {
        let cocoa_params = settings.borrow();

        if cocoa_params.label().is_some() {
            info!("settings label");
        }
        Ok(())
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
