/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub use tuple_list::{tuple_list as children_list, TupleList as ChildrenList};

use crate::utils::outlet::OutletType;

/*
pub trait HasChildren<CHILDREN>
where
    CHILDREN: ChildrenList,
{
    fn children(&self) -> &CHILDREN;
    fn mut_children(&self) -> &CHILDREN;
}*/

pub trait ChildOf<TY, OL: OutletType>: Sized {
    fn setup(&mut self, parent: &TY);
    fn connect(&self, parent: &TY);
    fn disconnect(&self);
}





impl<TY, OL: OutletType> ChildOf<TY, OL> for () {
    fn setup(&mut self, _parent: &TY) {}

    fn connect(&self, _parent: &TY) {}

    fn disconnect(&self) {}
}

impl<TY, OL: OutletType, Head, Tail> ChildOf<TY, OL> for (Head, Tail)
where
    Head: ChildOf<TY, OL>,
    Tail: ChildOf<TY, OL> + ChildrenList,
{
    fn setup(&mut self, parent: &TY) {
        self.0.setup(parent);
        self.1.setup(parent);
    }

    fn connect(&self, parent: &TY) {
        self.0.connect(parent);
        self.1.connect(parent);
    }

    fn disconnect(&self) {
        self.0.disconnect();
        self.1.disconnect();
    }
}
