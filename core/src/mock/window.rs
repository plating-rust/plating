/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::mock::traits::MockChildOf;
use crate::utils::children::{ChildOf, ChildrenList};
use crate::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use crate::widgets::{Widget, Window, WindowOutlet};
use crate::PlatingResult;


//TODO: fully automate
#[derive(Debug, Default, Clone, Hash)]
pub struct MockWindowOutlet<MAIN, MENU>
where
    MAIN: ChildrenList,
    MENU: ChildrenList,
{
    pub children: MAIN,
    pub menu:     MENU,
}


impl<MAIN, MENU> From<WindowOutlet<MAIN, MENU>> for MockWindowOutlet<MAIN, MENU>
where
    MAIN: ChildrenList,
    MENU: ChildrenList,
{
    fn from(app: WindowOutlet<MAIN, MENU>) -> Self {
        Self {
            children: app.children,
            menu:     app.menu,
        }
    }
}

impl<MAIN, MENU> Outlet<ChildrenOutlet> for MockWindowOutlet<MAIN, MENU>
where
    MAIN: ChildrenList,
    MENU: ChildrenList,
{
    type Children = MAIN;

    fn get(&self) -> &Self::Children {
        &self.children
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}

impl<MAIN, MENU> Outlet<MenuOutlet> for MockWindowOutlet<MAIN, MENU>
where
    MAIN: ChildrenList,
    MENU: ChildrenList,
{
    type Children = MENU;

    fn get(&self) -> &Self::Children {
        &self.menu
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.menu
    }
}

impl<MAIN, MENU> OutletHolder for MockWindowOutlet<MAIN, MENU>
where
    MAIN: ChildrenList,
    MENU: ChildrenList,
{
}
/*
impl<CHILDREN, MENU> Native<Cocoa> for MockWindowOutlet<MAIN_CHILDREN, MENU_CHILDREN>
where
    MAIN_CHILDREN: ChildrenList,
    MENU_CHILDREN: ChildrenList,
{ }*/



//TODO: end fully automate

pub struct MockWindowWidget<OUTLET>
where
    OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
{
    outlet:   OUTLET,
    _private: crate::Private, // Creation is limited to our constructors
}

pub trait MockWindow<OUTLET>
where
    OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
    Self: Sized,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>
    where
        <OUTLET as Outlet<MenuOutlet>>::Children: MockChildOf<Self, MenuOutlet>,
        <OUTLET as Outlet<ChildrenOutlet>>::Children: MockChildOf<Self, ChildrenOutlet>;
}

impl<OUTLET> MockWindow<OUTLET> for MockWindowWidget<OUTLET>
where
    OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>
    where
        <OUTLET as Outlet<MenuOutlet>>::Children: MockChildOf<Self, MenuOutlet>,
        <OUTLET as Outlet<ChildrenOutlet>>::Children: MockChildOf<Self, ChildrenOutlet>,
    {
        let result = Self {
            outlet,
            _private: crate::Private {},
        };
        <OUTLET as Outlet<MenuOutlet>>::get(&result.outlet).connect(&result);
        <OUTLET as Outlet<ChildrenOutlet>>::get(&result.outlet).connect(&result);

        Ok(result)
    }
}

impl<OUTLET> Widget<OUTLET> for MockWindowWidget<OUTLET>
where
    OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
{
    fn outlet(&self) -> &OUTLET {
        &self.outlet
    }
}

impl<OUTLET> Window<OUTLET> for MockWindowWidget<OUTLET>
where
    OUTLET: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
    Self: Sized,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>
    where
        <OUTLET as Outlet<MenuOutlet>>::Children: ChildOf<Self, MenuOutlet>,
        <OUTLET as Outlet<ChildrenOutlet>>::Children: ChildOf<Self, ChildrenOutlet>,
    {
        <Self as MockWindow<OUTLET>>::new(outlet)
    }
}
