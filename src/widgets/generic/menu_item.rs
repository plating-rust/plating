/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{Child, MenuChildren, NativeWidget, System};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct MenuItemParameters {
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_hidden: Option<bool>,
}

pub trait MenuItemHandlerTrait {
    //todo:
    //clicking on item
    //focusing on item
}

pub trait NativeMenuItem<S: System>:
    NativeWidget<S, PARAMS = S::MenuItemParameterType>
    + MenuItemHandlerTrait
    + Child<S::MenuType, MenuChildren<S>, S>
{
}

/*
#[derive(Debug)]
pub struct MenuItem<S: System> {
    /// stores the underlying native widget.
    /// Most functions like `apply` are just forwarded to this.
    native: S::MenuItemType,
}

impl<S: System> Widget for MenuItem<S> {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = MenuItemParameters;
}
impl<S: System> WidgetHolder for MenuItem<S> {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl<S: System> GenericWidget<S> for MenuItem<S> {
    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self, S> {
        S::MenuItemType::new_with_name(name, settings)
            .map(|native| Self { native })
            .map_err(|native_error| native_error.into())
    }

    type NativeParameterType = <S::MenuItemType as Widget>::PARAMS;
    type NativeType = S::MenuItemType;
}*/
