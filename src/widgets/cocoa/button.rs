/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log::info;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaSystem, CocoaWindow};
use crate::widgets::events::{LifecycleHandler, ListenerType};
use crate::widgets::generic::{ButtonHandlerTrait, ButtonParameters, NativeButton};
use crate::widgets::utils::WidgetHolder;
use crate::widgets::{cocoa::error::CocoaResult, ButtonChildren};
use crate::widgets::{Child, System, Widget, WindowChildren};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CocoaButtonParameters {
    pub label: Option<String>,
}

impl From<ButtonParameters> for CocoaButtonParameters {
    fn from(generic: ButtonParameters) -> Self {
        CocoaButtonParameters {
            label: generic.label,
        }
    }
}

#[derive(Debug)]
pub struct CocoaButton {
    ///auto generate and add via derive(widgetParent(A, B    ))
    //main_outlet: Outlet<ButtonChildren, CocoaButton>,
    handle: CocoaDefaultHandleType,

    ///auto generate and add via derive(Widget)
    name: String,
}

impl Child<CocoaWindow, WindowChildren<CocoaSystem>, CocoaSystem> for CocoaButton {}

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

    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
    where
        T: Into<Self::PARAMS>,
    {
        let mut button = CocoaButton {
            name,
            handle: 0 as CocoaDefaultHandleType,
            //main_outlet: Outlet::<ButtonChildren>::default(),
        };
        button.apply(settings)?;
        Ok(button)
    }

    fn apply<T>(&mut self, settings: T) -> CocoaResult<()>
    where
        T: Into<Self::PARAMS>,
    {
        let settings = settings.into();
        if settings.label.is_some() {
            info!("settings label");
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

impl WidgetHolder for CocoaButton {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

impl NativeButton<CocoaSystem> for CocoaButton {}

impl LifecycleHandler for CocoaButton {
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

impl ButtonHandlerTrait for CocoaButton {
    fn set_exit_handler(&mut self, _handler: Box<impl FnMut()>) {
        todo!()
    }
    fn add_exit_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
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
