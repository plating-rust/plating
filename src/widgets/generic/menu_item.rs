/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

 use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{NativeWidget, GenericWidget, Widget, WidgetHolder};
use crate::widgets::native::{NativeMenuItem, NativeMenuItemParameters};
use crate::PlatingResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct MenuItemParameters {
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_hidden: Option<bool>,
}

#[derive(Debug)]
pub struct MenuItem {
    /// stores the underlying native widget.
    /// Most functions like `apply` are just forwarded to this.
    native: NativeMenuItem,
}

impl Widget for MenuItem {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = MenuItemParameters;
}
impl WidgetHolder for MenuItem {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl GenericWidget for MenuItem {
    type NativeType = NativeMenuItem;
    type NativeParameterType = NativeMenuItemParameters;
    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self> {
        NativeMenuItem::new_with_name(name, settings)
            .map(|native| Self { native })
            .map_err(|native_error| native_error.into())
    }
}