/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::utils::children::ChildOf;
use crate::utils::outlet::OutletType;

//TODO: autogenerate
pub trait MockChildOf<TY, OL>
where
    OL: OutletType,
    Self: Sized,
{
    fn connect(&self, parent: &TY);
    fn disconnect(&self);
}

//TODO: autogenerate
impl<CHILD, TY, OL> MockChildOf<TY, OL> for CHILD
where
    CHILD: ChildOf<TY, OL>,
    OL: OutletType,
{
    fn connect(&self, parent: &TY) {
        <Self as ChildOf<TY, OL>>::connect(self, parent);
    }

    fn disconnect(&self) {
        <Self as ChildOf<TY, OL>>::disconnect(self);
    }
}
