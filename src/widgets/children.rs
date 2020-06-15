/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Contains the generic definitions for what widgets type can the children of a given type.
//! 

use crate::widgets::generic::{
    Button,
    Window,
    Menu,
    MenuItem,
};
use crate::widgets::native::{
    NativeButton,
    NativeWindow,
    NativeMenu, NativeMenuItem,
    NativeRoot,
};
use crate::widgets::{OutletAdapter, Child, GenericWidget, NativeWidget, WidgetHolder};

/// Helper enum to differentiate between native and generic widget types
#[derive(Debug)]
pub enum WidgetType<GenericType, NativeType>
where
    GenericType: GenericWidget,
    NativeType: NativeWidget,
{
    GENERIC(GenericType),
    NATIVE(NativeType),

}
impl<GenericType, NativeType> WidgetHolder for WidgetType<GenericType, NativeType>
where
    GenericType: GenericWidget,
    NativeType: NativeWidget,
{
    fn name(&self) -> &str {
        match self {
            Self::GENERIC(generic) => generic.name(),
            Self::NATIVE(native) => native.name(),
        }
    }
}
impl<GenericType, NativeType, ParentType, ChildType> Child<ParentType, ChildType> for WidgetType<GenericType, NativeType>
where
    GenericType: GenericWidget,
    NativeType: NativeWidget + Child<ParentType, ChildType>,
    ParentType: NativeWidget + OutletAdapter<ChildType>,
    ChildType: WidgetHolder,
{
    fn adding_to(&self, parent: &ParentType::ParentData) {
        match self {
            Self::GENERIC(generic) => panic!(), //todo: maybe support in future?
            Self::NATIVE(native) => native.adding_to(parent),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonChildren {
    BUTTON(WidgetType<Button, NativeButton>),
}
impl From<Button> for ButtonChildren {
    fn from(button: Button) -> Self {
        ButtonChildren::BUTTON(WidgetType::GENERIC(button))
    }
}
impl From<NativeButton> for ButtonChildren {
    fn from(button: NativeButton) -> Self {
        ButtonChildren::BUTTON(WidgetType::NATIVE(button))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl WidgetHolder for ButtonChildren {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowChildren {
    BUTTON(WidgetType<Button, NativeButton>),
}
impl From<Button> for WindowChildren {
    fn from(button: Button) -> Self {
        WindowChildren::BUTTON(WidgetType::GENERIC(button))
    }
}
impl From<NativeButton> for WindowChildren {
    fn from(button: NativeButton) -> Self {
        WindowChildren::BUTTON(WidgetType::NATIVE(button))
    }
}
impl Child<NativeWindow, WindowChildren> for WindowChildren{
    fn adding_to(&self, parent: &<NativeWindow as OutletAdapter<WindowChildren>>::ParentData) {
        match self {
            Self::BUTTON(button) => button.adding_to(parent)
        }
    }
}
/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl WidgetHolder for WindowChildren {
    fn name(&self) -> &str {
        match self {
            Self::BUTTON(button) => button.name(),
        }
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum RootChildren {
    WINDOW(WidgetType<Window, NativeWindow>),
}
impl From<Window> for RootChildren {
    fn from(window: Window) -> Self {
        RootChildren::WINDOW(WidgetType::GENERIC(window))
    }
}
impl From<NativeWindow> for RootChildren {
    fn from(window: NativeWindow) -> Self {
        RootChildren::WINDOW(WidgetType::NATIVE(window))
    }
}
impl WidgetHolder for RootChildren {
    fn name(&self) -> &str {
        match self {
            Self::WINDOW(window) => window.name(),
        }
    }
}
impl Child<NativeRoot, RootChildren> for RootChildren {
    fn adding_to(&self, parent: &<NativeRoot as OutletAdapter<RootChildren>>::ParentData) {
        match self {
            Self::WINDOW(button) => button.adding_to(parent)
        }
    }
}

// todo auto generate via derive(widgetParent(BUTTON, B    ))
#[derive(Debug)]
#[non_exhaustive]
pub enum MenuChildren
{
    ITEM(WidgetType<MenuItem, NativeMenuItem>), //todo
    MENU(WidgetType<Menu, NativeMenu>),
}
impl From<Menu> for MenuChildren {
    fn from(menu: Menu) -> Self {
        MenuChildren::MENU(WidgetType::GENERIC(menu))
    }
}
impl From<NativeMenu> for MenuChildren {
    fn from(menu: NativeMenu) -> Self {
        MenuChildren::MENU(WidgetType::NATIVE(menu))
    }
}
impl From<MenuItem> for MenuChildren {
    fn from(menu_item: MenuItem) -> Self {
        MenuChildren::ITEM(WidgetType::GENERIC(menu_item))
    }
}
impl From<NativeMenuItem> for MenuChildren {
    fn from(menu_item: NativeMenuItem) -> Self {
        MenuChildren::ITEM(WidgetType::NATIVE(menu_item))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl WidgetHolder for MenuChildren {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
            Self::ITEM(item) => item.name(),
        }
    }
}
impl Child<NativeMenu, MenuChildren> for MenuChildren {
    fn adding_to(&self, parent: &<NativeMenu as OutletAdapter<Self>>::ParentData) {
        match self {
            Self::MENU(menu) => <Child<NativeMenu, MenuChildren>>::adding_to(menu, parent),
            Self::ITEM(item) => item.adding_to(parent)
        }
    }
}


#[derive(Debug)]
#[non_exhaustive]
pub enum MainMenuChildren
{ 
    MENU(WidgetType<Menu, NativeMenu>),
}
impl From<Menu> for MainMenuChildren {
    fn from(menu: Menu) -> Self {
        Self::MENU(WidgetType::GENERIC(menu))
    }
}
impl From<NativeMenu> for MainMenuChildren {
    fn from(menu: NativeMenu) -> Self {
        Self::MENU(WidgetType::NATIVE(menu))
    }
}

/// todo auto generate via derive(widgetParent(BUTTON, B    ))
impl WidgetHolder for MainMenuChildren {
    fn name(&self) -> &str {
        match self {
            Self::MENU(menu) => menu.name(),
        }
    }
}
impl Child<NativeWindow, MainMenuChildren> for MainMenuChildren
{
    fn adding_to(&self, parent: &<NativeWindow as OutletAdapter<MainMenuChildren> >::ParentData) {
        match self {
            Self::MENU(menu) => <Child<NativeWindow, MainMenuChildren>>::adding_to(menu, parent),
        }
    }
}
