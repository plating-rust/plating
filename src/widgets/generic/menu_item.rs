/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::{Child, MenuChildren, System, Widget};

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
    Widget<S, PARAMS = S::MenuItemParameterType>
    + MenuItemHandlerTrait
    + Child<S::MenuType, MenuChildren<S>, S>
{
}
