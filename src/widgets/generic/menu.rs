/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Connectable, Named};
use crate::widgets::window::MainMenuChildren;
use crate::widgets::{default_system, System, Widget};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct MenuParameters {
    pub title: Option<String>,
}

pub trait MenuHandlerTrait {
    //todo:
    //opening menu
    //closing menu
}

pub trait NativeMenu<S: System>:
    Widget<S, PARAMS = S::MenuParameterType>
    + MenuHandlerTrait
    + Outlet<MenuChildren<S>, S>
    + Child<S::MenuType, MenuChildren<S>, S>
    + Child<S::WindowType, MainMenuChildren<S>, S>
{
}

// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum MenuChildren<S: System = default_system> {
    ITEM(S::MenuItemType), //todo
    MENU(S::MenuType),
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> Named for MenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
            Self::ITEM(item) => item.name(),
        }
    }
}

impl<S: System> Connectable for MenuChildren<S> {
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

impl<S: System> Child<S::MenuType, MenuChildren<S>, S> for MenuChildren<S> {
    fn adding_to_parent(&mut self, parent: &<S::MenuType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::MenuType, MenuChildren<S>, S>>::adding_to_parent(menu, parent)
            }
            Self::ITEM(item) => item.adding_to_parent(parent),
        }
    }

    fn removing_from_parent(&mut self) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::MenuType, MenuChildren<S>, S>>::removing_from_parent(menu)
            }
            Self::ITEM(item) => item.removing_from_parent(),
        }
    }
    fn added(&self) -> bool {
        match self {
            Self::MENU(menu) => <dyn Child<S::MenuType, MenuChildren<S>, S>>::added(menu),
            Self::ITEM(item) => item.added(),
        }
    }
}
