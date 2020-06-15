/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::native::{NativeButton, NativeButtonParameters};
use crate::widgets::{
    ButtonChildren, ChildrenHolder, GenericWidget, NativeWidget, Widget, WidgetHolder, OutletAdapter,
};
use crate::PlatingResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ButtonParameters {
    pub label: Option<String>,
}

#[derive(Debug)]
pub struct Button {
    native: NativeButton,
}
// auto generate impl via derive(widgetParent(A, B    ))
/*
impl OutletAdapter<ButtonChildren> for Button {
    type AdditionResult = PlatingResult<()>;

    fn children(&self) -> &[ChildrenHolder<ButtonChildren>] {
        self.native.children()
    }
    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<ButtonChildren>,
    {
        self.native
            .add_child(child)
            .map_err(|native_error| native_error.into())
    }
}*/
//auto generate impl via derive(widgetParent(A, B    ))
impl WidgetHolder for Button {
    fn name(&self) -> &str {
        self.native.name()
    }
}
impl Widget for Button {
    type PARAMS = ButtonParameters;
}
impl GenericWidget for Button {
    type NativeType = NativeButton;
    type NativeParameterType = NativeButtonParameters;

    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self> {
        NativeButton::new_with_name(name, settings)
            .map(|native| Button { native })
            .map_err(|native_error| native_error.into())
    }
}
