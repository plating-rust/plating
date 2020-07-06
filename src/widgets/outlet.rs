/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::utils::{Named, OutletIterator, WidgetPointer};
use crate::widgets::{default_system, System};
use std::rc::Rc;

type ChildIter<'a, CHILD> = std::iter::FilterMap<
    std::slice::Iter<'a, WidgetPointer<CHILD>>,
    fn(&WidgetPointer<CHILD>) -> Option<Rc<CHILD>>,
>;

fn get_obj<CHILD: Named>(obj: &WidgetPointer<CHILD>) -> Option<Rc<CHILD>> {
    obj.get()
}

pub trait Outlet<CHILD, S = default_system>
where
    CHILD: Named,
    S: System,
{
    type ErrorType: Into<crate::error::PlatingError<S>>;
    type ParentData;

    fn iter<'a>(&'a self) -> OutletIterator<'a, CHILD>;

    fn get_by_name<STR: std::borrow::Borrow<str>>(&self, name: STR) -> Option<std::rc::Rc<CHILD>> {
        self.iter()
            .find(|obj: &Rc<CHILD>| obj.name() == name.borrow())
    }

    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
    fn as_slice(&self) -> &[WidgetPointer<CHILD>];
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    //todo fn remove_remnants(&mut self);

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<CHILD>;

    fn insert_child<T>(
        &mut self,
        index: usize,
        child: T,
    ) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<CHILD>;

    //todo: removing children
    // - by index
    // - by name
    // - by Rc<CHILD>
    // - by predicate
}
