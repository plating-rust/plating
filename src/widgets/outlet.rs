/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::utils::{Named, WidgetPointer};
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

    fn children(&self) -> &[WidgetPointer<CHILD>];

    fn get_by_name<STR: std::borrow::Borrow<str>>(&self, name: STR) -> Option<std::rc::Rc<CHILD>> {
        self.child_iter()
            .find(|obj: &Rc<CHILD>| obj.name() == name.borrow())
    }

    /// Convenience Function that gives you an iterator directly to the children instead of ChildrenHolder.
    ///
    /// This will also filter out dangling weak references beforehand.
    fn child_iter(&self) -> ChildIter<CHILD> {
        self.children()
            .iter()
            .filter_map(get_obj as fn(&WidgetPointer<CHILD>) -> Option<Rc<CHILD>>)
    }

    //todo fn remove_remnants(&mut self);

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<CHILD>;

    //todo: removing children
}
