/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::RootChildren;
use crate::widgets::{ default_system, System, OutletAdapter,
    ChildrenHolder, GenericWidget, NativeWidget, Widget, WidgetHolder,
};
use crate::PlatingResult;

//todo: move
pub trait RootWidgetTrait<S: System> {
    fn run(&self) -> std::result::Result<(), S::ErrorType>;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)] //required in cached version
pub struct RootParameters {}

#[derive(Debug)]
pub struct Root<S: System = default_system> {
    //derive for 'derive(GenericWidget)'
    native: S::RootType,
}
impl<S: System> Root<S> {
    pub fn run(&self) -> PlatingResult<(), S> {
        self.native
            .run()
            .map_err(|native_error| native_error.into())
    }
}
impl<S: System> Widget for Root<S> {
    type PARAMS = RootParameters;
}

impl<S: System> WidgetHolder for Root<S> {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl<S: System> GenericWidget<S> for Root<S> {

    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self, S> {
        S::RootType::new_with_name(name, settings)
            .map(|native| Root { native })
            .map_err(|native_error| native_error.into())
    }

    type NativeParameterType = <S::RootType as Widget>::PARAMS;
    type NativeType = S::RootType;
}

//derive for 'derive(GenericWidget)'
impl<S: System> OutletAdapter<RootChildren<S>, S> for Root<S> {
    type ErrorType = crate::error::PlatingError<S>;
    type ParentData = <S::RootType as OutletAdapter<RootChildren<S>, S>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<RootChildren<S>>] {
        self.native.children()
    }

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<RootChildren<S>>,
    {
        self.native
            .add_child(child.into())
            .map_err(|native_error| native_error.into())
    }
}
