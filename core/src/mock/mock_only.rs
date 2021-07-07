/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use super::MockWindowWidget;
use crate::utils::children::ChildOf;
use crate::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use crate::widgets::Widget;
use crate::PlatingResult;


#[derive(Debug, Default)]
pub struct MockOnlyOutlet {}
impl OutletHolder for MockOnlyOutlet {}

pub struct MockOnlyWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    outlet:   OUTLET,
    _private: crate::Private, // Creation is limited to our constructors
}

pub trait MockOnly<OUTLET>
where
    OUTLET: OutletHolder,
    Self: Sized,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>;
}

impl<OUTLET> MockOnly<OUTLET> for MockOnlyWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self> {
        let result = Self {
            outlet,
            _private: crate::Private {},
        };
        Ok(result)
    }
}

impl<OUTLET> Widget<OUTLET> for MockOnlyWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    fn outlet(&self) -> &OUTLET {
        &self.outlet
    }
}

impl<OUTLET1, OUTLET2> ChildOf<MockWindowWidget<OUTLET1>, ChildrenOutlet>
    for MockOnlyWidget<OUTLET2>
where
    OUTLET1: OutletHolder + Outlet<MenuOutlet> + Outlet<ChildrenOutlet>,
    OUTLET2: OutletHolder,
{
    fn setup(&mut self, _parent: &MockWindowWidget<OUTLET1>) {
        //todo!()
    }

    fn connect(&self, _parent: &MockWindowWidget<OUTLET1>) {
        //todo!()
    }

    fn disconnect(&self) {
        //todo!()
    }
}
