/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains the generic definitions for what widgets type can the children of a given type.
//!

use crate::widgets::generic::{Button, Menu, MenuItem, Window};

use crate::widgets::{
    default_system, Child, GenericWidget, NativeWidget, OutletAdapter, System, WidgetHolder,
};

/// Helper enum to differentiate between native and generic widget types
#[derive(Debug)]
pub enum WidgetType<GenericType, NativeType, S = default_system>
where
    GenericType: GenericWidget<S>,
    NativeType: NativeWidget<S>,
    S: System,
{
    GENERIC(GenericType),
    NATIVE(NativeType),

    //todo: not happy about this!
    _N(std::marker::PhantomData<*const S>),
}
impl<GenericType, NativeType, S> WidgetHolder for WidgetType<GenericType, NativeType, S>
where
    GenericType: GenericWidget<S>,
    NativeType: NativeWidget<S>,
    S: System,
{
    fn name(&self) -> &str {
        match self {
            Self::GENERIC(generic) => generic.name(),
            Self::NATIVE(native) => native.name(),
            Self::_N(_) => panic!(),
        }
    }
}
impl<GenericType, NativeType, ParentType, ChildType, S> Child<ParentType, ChildType, S>
    for WidgetType<GenericType, NativeType, S>
where
    GenericType: GenericWidget<S>,
    NativeType: NativeWidget<S> + Child<ParentType, ChildType, S>,
    ParentType: NativeWidget<S> + OutletAdapter<ChildType, S>,
    ChildType: WidgetHolder,
    S: System,
{
    fn adding_to(&self, parent: &ParentType::ParentData) {
        match self {
            Self::GENERIC(generic) => todo!(), //todo: maybe support in future?
            Self::NATIVE(native) => native.adding_to(parent),
            Self::_N(_) => panic!(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonChildren<S: System = default_system> {
    BUTTON(WidgetType<Button<S>, S::ButtonType, S>),
}
impl<S: System> From<Button<S>> for ButtonChildren<S> {
    fn from(button: Button<S>) -> Self {
        ButtonChildren::BUTTON(WidgetType::GENERIC(button))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for ButtonChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowChildren<S: System = default_system> {
    BUTTON(WidgetType<Button<S>, S::ButtonType, S>),
}
impl<S: System> From<Button<S>> for WindowChildren<S> {
    fn from(button: Button<S>) -> Self {
        WindowChildren::BUTTON(WidgetType::GENERIC(button))
    }
}

impl<S: System> Child<S::WindowType, WindowChildren<S>, S> for WindowChildren<S> {
    fn adding_to(
        &self,
        parent: &<S::WindowType as OutletAdapter<WindowChildren<S>, S>>::ParentData,
    ) {
        match self {
            Self::BUTTON(button) => button.adding_to(parent),
        }
    }
}
/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for WindowChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum RootChildren<S: System = default_system> {
    WINDOW(WidgetType<Window<S>, S::WindowType, S>),
}
impl<S: System> From<Window<S>> for RootChildren<S> {
    fn from(window: Window<S>) -> Self {
        RootChildren::WINDOW(WidgetType::GENERIC(window))
    }
}

impl<S: System> WidgetHolder for RootChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::WINDOW(window) => window.name(),
        }
    }
}
impl<S: System> Child<S::RootType, RootChildren<S>, S> for RootChildren<S> {
    fn adding_to(&self, parent: &<S::RootType as OutletAdapter<RootChildren<S>, S>>::ParentData) {
        match self {
            Self::WINDOW(button) => button.adding_to(parent),
        }
    }
}

// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum MenuChildren<S: System = default_system> {
    ITEM(WidgetType<MenuItem<S>, S::MenuItemType, S>), //todo
    MENU(WidgetType<Menu<S>, S::MenuType, S>),
}
impl<S: System> From<Menu<S>> for MenuChildren<S> {
    fn from(menu: Menu<S>) -> Self {
        MenuChildren::MENU(WidgetType::GENERIC(menu))
    }
}

impl<S: System> From<MenuItem<S>> for MenuChildren<S> {
    fn from(menu_item: MenuItem<S>) -> Self {
        MenuChildren::ITEM(WidgetType::GENERIC(menu_item))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for MenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
            Self::ITEM(item) => item.name(),
        }
    }
}
impl<S: System> Child<S::MenuType, MenuChildren<S>, S> for MenuChildren<S> {
    fn adding_to(&self, parent: &<S::MenuType as OutletAdapter<Self, S>>::ParentData) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::MenuType, MenuChildren<S>, S>>::adding_to(menu, parent)
            }
            Self::ITEM(item) => item.adding_to(parent),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren<S: System = default_system> {
    MENU(WidgetType<Menu<S>, S::MenuType, S>),
}
impl<S: System> From<Menu<S>> for MainMenuChildren<S> {
    fn from(menu: Menu<S>) -> Self {
        Self::MENU(WidgetType::GENERIC(menu))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl<S: System> WidgetHolder for MainMenuChildren<S> {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
        }
    }
}
impl<S: System> Child<S::WindowType, MainMenuChildren<S>, S> for MainMenuChildren<S> {
    fn adding_to(
        &self,
        parent: &<S::WindowType as OutletAdapter<MainMenuChildren<S>, S>>::ParentData,
    ) {
        match self {
            Self::MENU(menu) => {
                <dyn Child<S::WindowType, MainMenuChildren<S>, S>>::adding_to(menu, parent)
            }
        }
    }
}
