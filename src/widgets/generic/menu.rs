/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::generic::MainMenuChildren;
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Named};
use crate::widgets::{default_system, System, Widget};

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
impl<S: System> Child<S::MenuType, MenuChildren<S>, S> for MenuChildren<S> {
    fn adding_to(&self, parent: &<S::MenuType as Outlet<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::MenuType, MenuChildren<S>, S>>::adding_to(menu, parent)
            }
            Self::ITEM(item) => item.adding_to(parent),
        }
    }
}
