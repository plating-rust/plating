/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Connectable, Identity, Parameters};
use crate::widgets::window::MainMenuChildren;
use crate::widgets::{default_system, System, Widget};
use plating_macros::Identifiable;

pub trait MenuParameters: Parameters {
    fn label(&self) -> &Option<String>;

    fn set_label(&mut self, label: String) -> &mut Self;
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self;
    fn unset_label(&mut self) -> &mut Self;
}

pub trait MenuHandlerTrait {
    //todo:
    //opening menu
    //closing menu
}

pub trait Menu<S: System + ?Sized>:
    Widget<S, PARAMS = S::MenuParameterType>
    + MenuHandlerTrait
    + Outlet<MenuChildren<S>, S>
    + Child<S::MenuType, MenuChildren<S>, S>
    + Child<S::WindowType, MainMenuChildren<S>, S>
{
}

// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug, Identifiable)]
#[non_exhaustive]
pub enum MenuChildren<S: System + ?Sized = default_system> {
    ITEM(S::MenuItemType), //todo
    MENU(S::MenuType),
}

impl<S: System + ?Sized> Connectable for MenuChildren<S> {
    fn connecting(&mut self) {
        match self {
            Self::MENU(menu) => menu.connecting(),
            Self::ITEM(item) => item.connecting(),
        }
    }

    fn disconnecting(&mut self) {
        match self {
            Self::MENU(menu) => menu.disconnecting(),
            Self::ITEM(item) => item.disconnecting(),
        }
    }

    fn connected(&self) -> bool {
        match self {
            Self::MENU(menu) => menu.connected(),
            Self::ITEM(item) => item.connected(),
        }
    }
}

impl<S: System + ?Sized> Child<S::MenuType, MenuChildren<S>, S> for MenuChildren<S> {
    fn adding_to_parent(&mut self, parent: &<S::MenuType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => <dyn Child<S::MenuType, Self, S>>::adding_to_parent(menu, parent),
            Self::ITEM(item) => item.adding_to_parent(parent),
        }
    }

    fn removing_from_parent(&mut self) {
        match self {
            Self::MENU(menu) => <dyn Child<S::MenuType, Self, S>>::removing_from_parent(menu),
            Self::ITEM(item) => item.removing_from_parent(),
        }
    }
    fn added(&self) -> bool {
        match self {
            Self::MENU(menu) => <dyn Child<S::MenuType, Self, S>>::added(menu),
            Self::ITEM(item) => item.added(),
        }
    }
}
