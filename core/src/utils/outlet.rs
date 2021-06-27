/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::utils::children::ChildrenList;

pub trait OutletType {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct ChildrenOutlet {}
impl OutletType for ChildrenOutlet {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct MenuOutlet {}
impl OutletType for MenuOutlet {}

pub trait Outlet<OUTLET>
where
    OUTLET: OutletType,
{
    type Children: ChildrenList;

    //todo: move setup() here?

    fn get(&self) -> &Self::Children;
    fn get_mut(&mut self) -> &mut Self::Children;
}

pub trait OutletHolder {}
