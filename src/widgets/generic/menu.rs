/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{Child, MainMenuChildren, MenuChildren, NativeWidget, OutletAdapter, System};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct MenuParameters {
    pub title: Option<String>,
}

pub trait MenuHandlerTrait {
    //todo:
    //opening menu
    //closing menu
}

pub trait NativeMenu<S: System>:
    NativeWidget<S, PARAMS = S::MenuParameterType>
    + MenuHandlerTrait
    + OutletAdapter<MenuChildren<S>, S>
    + Child<S::MenuType, MenuChildren<S>, S>
    + Child<S::WindowType, MainMenuChildren<S>, S>
{
}

/*
#[derive(Debug)]
pub struct Menu<S: System> {
    /// stores the underlying native widget.
    /// Most functions like `apply` are forwarded to this internal, native type.
    native: S::MenuType,
}
impl<S: System> Widget for Menu<S> {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = MenuParameters;
}
impl<S: System> WidgetHolder for Menu<S> {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl<S: System> GenericWidget<S> for Menu<S> {
    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self, S> {
        S::MenuType::new_with_name(name, settings)
            .map(|native| Self { native })
            .map_err(|native_error| native_error.into())
    }
    type NativeParameterType = <S::MenuType as Widget>::PARAMS;
    type NativeType = S::MenuType;
}

impl<S: System> OutletAdapter<MenuChildren<S>, S> for Menu<S> {
    type ErrorType = crate::error::PlatingError<S>;
    type ParentData = <S::MenuType as OutletAdapter<MenuChildren<S>, S>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<MenuChildren<S>>] {
        self.native.children()
    }

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<MenuChildren<S>>,
    {
        self.native
            .add_child(child.into())
            .map_err(|native_error| native_error.into())
    }
}
*/
