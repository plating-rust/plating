/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub use tuple_list::{tuple_list as children_list, TupleList as ChildrenList};

/*
pub trait HasChildren {
    type Children: ChildrenList;

    fn children(&self) -> &Self::Children;
    fn children_mut(&mut self) -> &mut Self::Children;
}

pub trait HasMenu {
    type Menu: ChildrenList;

    fn menu(&self) -> &Self::Menu;
    fn menu_mut(&mut self) -> &mut Self::Menu;
}*/

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

pub trait ChildOf<T>: Sized {
    fn setup(&mut self, parent: &T);
    fn connect(&self, parent: &T);
    fn disconnect(&self);
}

impl<T> ChildOf<T> for () {
    fn setup(&mut self, _parent: &T) {}

    fn connect(&self, _parent: &T) {}

    fn disconnect(&self) {}
}

impl<T, Head, Tail> ChildOf<T> for (Head, Tail)
where
    Head: ChildOf<T>,
    Tail: ChildOf<T> + ChildrenList,
{
    fn setup(&mut self, parent: &T) {
        self.0.setup(parent);
        self.1.setup(parent);
    }

    fn connect(&self, parent: &T) {
        self.0.connect(parent);
        self.1.connect(parent);
    }

    fn disconnect(&self) {
        self.0.disconnect();
        self.1.disconnect();
    }
}
