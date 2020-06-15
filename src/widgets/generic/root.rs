/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::native::{NativeRoot, NativeRootParameters};
use crate::widgets::RootChildren;
use crate::widgets::{ OutletAdapter,
    ChildrenHolder, GenericWidget, NativeWidget, Widget, WidgetHolder,
};
use crate::error::PlatingError;
use crate::PlatingResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)] //required in cached version
pub struct RootParameters {}

#[derive(Debug)]
pub struct Root {
    //derive for 'derive(GenericWidget)'
    native: NativeRoot,
}
impl Root {
    pub fn run(&self) -> PlatingResult<()> {
        self.native
            .run()
            .map_err(|native_error| native_error.into())
    }
}
impl Widget for Root {
    type PARAMS = RootParameters;
}

impl WidgetHolder for Root {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl GenericWidget for Root {
    type NativeType = NativeRoot;
    type NativeParameterType = NativeRootParameters;

    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self> {
        NativeRoot::new_with_name(name, settings)
            .map(|native| Root { native })
            .map_err(|native_error| native_error.into())
    }
}

//derive for 'derive(GenericWidget)'
impl OutletAdapter<RootChildren> for Root {
    type AdditionResult = PlatingResult<()>;
    type ParentData = <NativeRoot as OutletAdapter<RootChildren>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<RootChildren>] {
        self.native.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<RootChildren>,
    {
        self.native
            .add_child(child.into())
            .map_err(|native_error| native_error.into())
    }
}
