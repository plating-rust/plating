/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::menu::MenuChildren;
use crate::widgets::utils::{Child, Parameters};
use crate::widgets::{System, Widget};

pub trait MenuItemParameters: Parameters {
    fn label(&self) -> &Option<String>;

    fn set_label(&mut self, label: String) -> &mut Self;
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self;
    fn unset_label(&mut self) -> &mut Self;

    fn enabled(&self) -> &Option<bool>;

    fn set_enabled(&mut self, label: bool) -> &mut Self;
    fn set_enabled_optionally(&mut self, label: Option<bool>) -> &mut Self;
    fn unset_enabled(&mut self) -> &mut Self;
}

pub trait MenuItemHandlerTrait {
    //todo:
    //clicking on item
    //focusing on item
}

pub trait MenuItem<S: System + ?Sized>:
    Widget<S, PARAMS = S::MenuItemParameterType>
    + MenuItemHandlerTrait
    + Child<S::MenuType, MenuChildren<S>, S>
{
}
