/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{MenuChildren, ChildrenHolder, GenericWidget, NativeWidget, Widget, WidgetHolder, OutletAdapter};
use crate::widgets::native::{NativeMenu, NativeMenuParameters};
use crate::PlatingResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct MenuParameters {
    pub title: Option<String>,
}

#[derive(Debug)]
pub struct Menu {
    /// stores the underlying native widget.
    /// Most functions like `apply` are just forwarded to this.
    native: NativeMenu,
}
impl Widget for Menu {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = MenuParameters;
}
impl WidgetHolder for Menu {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl GenericWidget for Menu {
    type NativeType = NativeMenu;
    type NativeParameterType = NativeMenuParameters;
    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self> {
        NativeMenu::new_with_name(name, settings)
            .map(|native| Self { native })
            .map_err(|native_error| native_error.into())
    }
}

impl OutletAdapter<MenuChildren> for Menu {
    type AdditionResult = PlatingResult<()>;
    type ParentData = <NativeMenu as OutletAdapter<MenuChildren>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<MenuChildren>] {
        self.native.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<MenuChildren>,
    {
        self.native
            .add_child(child.into())
            .map_err(|native_error| native_error.into())
    }
}