/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing struct of the same Name [`Outlet`].

use std::rc::Rc;

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Named, WidgetPointer};
use crate::widgets::{System, Widget};

/// Outlets are a concepts for widgets to have children.
///
/// Widgets can have zero, one or more outlets. For example the [Window](crate::widgets::generic::window) Widget
/// has two outlets.
/// - one for storing all the content inside it
/// - the other one for storing the main menu
///
/// Both outlets are strongly typed so you cannot add a Button to the Main Menu outlet and you cannot add
/// a Menu Item to the content outlet of the window.
///
/// The way to interact with a Outlet is usually via the [WidgetParent](`crate::widgets::traits::WidgetParent`) trait.
/// Most Widgets usually have some sort of 'default' outlet. Those Widgets implement the WidgetParent trait themselves
/// and pass those function calls forward to an internal instance of Outlet. The aforementioned 'content' outlet is the main outlet.
///
/// Outlets store the children in vectors of [`ChildrenHolders`](crate::widgets::traits::ChildrenHolder).
/// Make sure to read up on them to understand ownership and memory management of plating
///
/// # Template Parameters
/// - `CHILD`: the kind of elements this Outlet can store.<br>
///     Usually realized by enums. See [`WindowChildren`](crate::widgets::WindowChildren) for an example.
///     <br><br>**Requirements**: Needs to implement [`WidgetHolder`] + `std::fmt::Debug`
///
#[derive(Debug, Serialize, Deserialize)]
pub struct OutletHolder<CHILD, Parent, S>
where
    CHILD: Named + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    ///Vector responsible for storing all the Children.
    ///
    /// Uses a [`ChildrenHolder`] instead of the children directly
    pub(crate) children: Vec<WidgetPointer<CHILD>>,

    _marker: std::marker::PhantomData<Parent>,
    _marker2: std::marker::PhantomData<S>,
}

//todo: implement fromIterator
//todo: implement Extend

impl<CHILD, Parent, S> Default for OutletHolder<CHILD, Parent, S>
where
    CHILD: Named + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    fn default() -> OutletHolder<CHILD, Parent, S> {
        Self {
            children: vec![],
            _marker: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
        }
    }
}
impl<CHILD, Parent, S> OutletHolder<CHILD, Parent, S>
where
    CHILD: Named + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    /// Returns the capacity of the internal vector.
    ///
    /// See [Vec::capacity]
    pub fn capacity(&self) -> usize {
        self.children.capacity()
    }
    /// Reserves space for the specified amount of *additional* children.
    ///
    /// See [Vec::reserve]
    pub fn reserve(&mut self, additional: usize) {
        self.children.reserve(additional)
    }
    /// Reserves space for the specified amount of children *in total*.
    ///
    /// See [Vec::reserve_exact]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.children.reserve_exact(additional)
    }
    /// Resizes the vector to fit the current amount of children.
    ///
    /// See [Vec::shrink_to_fit]
    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
    }

    pub fn as_slice(&self) -> &[WidgetPointer<CHILD>] {
        self.children.as_slice()
    }

    pub fn clear(&mut self) {
        self.children.clear()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn insert_child<T>(
        &mut self,
        index: usize,
        child: T,
        parent: &Parent::ParentData,
    ) -> std::result::Result<(), S::ErrorType>
    where
        T: Into<CHILD>,
    {
        let into_child = child.into();
        into_child.adding_to(parent);
        self.children
            .insert(index, WidgetPointer::Ours(Rc::new(into_child)));

        Ok(())
    }

    pub(crate) fn push_child<T>(
        &mut self,
        child: T,
        parent: &Parent::ParentData,
    ) -> std::result::Result<(), S::ErrorType>
    where
        T: Into<CHILD>,
    {
        let into_child = child.into();
        into_child.adding_to(parent);
        self.children.push(WidgetPointer::Ours(Rc::new(into_child)));

        Ok(())
    }

    //

    pub(crate) fn iter<'a>(&'a self) -> OutletIterator<'a, CHILD> {
        OutletIterator::new(self.children.iter())
    }
}

impl<CHILD, Parent, S> std::ops::Index<usize> for OutletHolder<CHILD, Parent, S>
where
    CHILD: Named + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    type Output = WidgetPointer<CHILD>;

    fn index(&self, index: usize) -> &WidgetPointer<CHILD> {
        &self.children[index]
    }
}

#[derive(Debug, Clone)]
pub struct OutletIterator<'a, CHILD>
where
    CHILD: Named,
{
    internal_iter: std::slice::Iter<'a, WidgetPointer<CHILD>>,
}
impl<'a, CHILD> OutletIterator<'a, CHILD>
where
    CHILD: Named,
{
    fn new(iter: std::slice::Iter<'a, WidgetPointer<CHILD>>) -> Self {
        Self {
            internal_iter: iter,
        }
    }
}

impl<'a, CHILD> Iterator for OutletIterator<'a, CHILD>
where
    CHILD: Named,
{
    type Item = Rc<CHILD>;

    #[inline]
    fn next(&mut self) -> Option<Rc<CHILD>> {
        let n = self.internal_iter.next();
        if let Some(pointer) = n {
            match pointer.get() {
                Some(p) => Some(p),
                None => self.next(),
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal_iter.size_hint()
    }
}

impl<'a, CHILD> DoubleEndedIterator for OutletIterator<'a, CHILD>
where
    CHILD: Named,
{
    #[inline]
    fn next_back(&mut self) -> Option<Rc<CHILD>> {
        let n = self.internal_iter.next_back();
        if let Some(pointer) = n {
            match pointer.get() {
                Some(p) => Some(p),
                None => self.next(),
            }
        } else {
            None
        }
    }
}

impl<'a, CHILD> ExactSizeIterator for OutletIterator<'_, CHILD> where CHILD: Named {}

impl<'a, CHILD> core::iter::FusedIterator for OutletIterator<'_, CHILD> where CHILD: Named {}
