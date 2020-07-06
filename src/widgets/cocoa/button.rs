/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::log::info;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::cocoa::{CocoaDefaultHandleType, CocoaSystem, CocoaWindow};
use crate::widgets::generic::ButtonParameters;
use crate::widgets::{cocoa::error::CocoaResult, ButtonChildren, WidgetType};
use crate::widgets::{Child, NativeWidget, System, Widget, WidgetHolder, WindowChildren};
use crate::{prelude::NativeButton, widgets::cocoa::delegates::CocoaWindowDelegate};

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

impl Widget for CocoaButton {
    type PARAMS = CocoaButtonParameters;
}
impl Child<CocoaWindow, WindowChildren<CocoaSystem>, CocoaSystem> for CocoaButton {}

impl From<CocoaButton> for ButtonChildren<CocoaSystem> {
    fn from(button: CocoaButton) -> Self {
        ButtonChildren::BUTTON(WidgetType::NATIVE(button))
    }
}

impl From<CocoaButton> for WindowChildren<CocoaSystem> {
    fn from(button: CocoaButton) -> Self {
        WindowChildren::BUTTON(WidgetType::NATIVE(button))
    }
}

impl NativeWidget<CocoaSystem> for CocoaButton {
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
}

impl WidgetHolder for CocoaButton {
    fn name(&self) -> &str {
        &self.name.as_str()
    }
}

impl NativeButton<CocoaSystem> for CocoaButton {}

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
