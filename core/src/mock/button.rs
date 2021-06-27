/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use super::MockWindowWidget;
use crate::utils::children::ChildOf;
use crate::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
use crate::widgets::{Button, Widget};
use crate::PlatingResult;

#[derive(Debug, Default)]
pub struct MockButtonOutlet {}
impl OutletHolder for MockButtonOutlet {}

pub struct MockButtonWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    outlet:   OUTLET,
    _private: crate::Private, // Creation is limited to our constructors
}

pub trait MockButton<OUTLET>
where
    OUTLET: OutletHolder,
    Self: Sized,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self>;
}

impl<OUTLET> MockButton<OUTLET> for MockButtonWidget<OUTLET>
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

impl<OUTLET> Widget<OUTLET> for MockButtonWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    fn outlet(&self) -> &OUTLET {
        &self.outlet
    }

    fn outlet_mut(&mut self) -> &mut OUTLET {
        &mut self.outlet
    }
}

impl<OUTLET> Button<OUTLET> for MockButtonWidget<OUTLET>
where
    OUTLET: OutletHolder,
{
    fn new(outlet: OUTLET) -> PlatingResult<Self> {
        <Self as MockButton<OUTLET>>::new(outlet)
    }
}

impl<OUTLET1, OUTLET2> ChildOf<MockWindowWidget<OUTLET1>, ChildrenOutlet>
    for MockButtonWidget<OUTLET2>
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
