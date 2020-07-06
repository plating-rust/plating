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
