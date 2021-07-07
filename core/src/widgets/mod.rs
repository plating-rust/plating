/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::utils::children::{ChildOf, ChildrenList};
use crate::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use crate::PlatingResult;


pub trait Widget<OUTLET>
where
    OUTLET: OutletHolder,
{
    fn outlet(&self) -> &OUTLET;
}

#[derive(Debug, Default)]
pub struct WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    pub children: CHILDREN,
    pub menu:     MENU,
}

impl<CHILDREN, MENU> OutletHolder for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
}

impl<CHILDREN, MENU> Outlet<MenuOutlet> for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
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


impl<CHILDREN, MENU> Outlet<ChildrenOutlet> for WindowOutlet<CHILDREN, MENU>
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    type Children = CHILDREN;

    fn get(&self) -> &Self::Children {
        &self.children
    }

    fn get_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}

pub trait Window<OUTLET>
where
    OUTLET: Outlet<MenuOutlet> + Outlet<ChildrenOutlet> + OutletHolder,
    Self: Sized + Widget<OUTLET>,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>
    where
        <OUTLET as Outlet<MenuOutlet>>::Children: ChildOf<Self, MenuOutlet>,
        <OUTLET as Outlet<ChildrenOutlet>>::Children: ChildOf<Self, ChildrenOutlet>;
}


#[derive(Debug, Default)]
pub struct ButtonOutlet {}
impl OutletHolder for ButtonOutlet {}


pub trait Button<OUTLET>
where
    OUTLET: OutletHolder,
    Self: Sized + Widget<OUTLET>,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>;
}


mod outlets {
    use crate::utils::outlet::OutletType;

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    pub struct ChildrenOutlet {}
    impl OutletType for ChildrenOutlet {}

    #[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
    pub struct MenuOutlet {}
    impl OutletType for MenuOutlet {}
}

pub mod prelude {
    pub use super::{Button, Window};
}
