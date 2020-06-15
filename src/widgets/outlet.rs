/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing struct of the same Name [`Outlet`].

use std::rc::Rc;

use crate::features::serde::{Deserialize, Serialize, SerializeTrait, DeserializeTrait};
use crate::widgets::{Child, NativeWidget, ChildrenHolder, WidgetHolder};
use crate::NativeResult;

type ChildIter<'a, CHILD> = std::iter::FilterMap<
    std::slice::Iter<'a, ChildrenHolder<CHILD>>,
    fn(&ChildrenHolder<CHILD>) -> Option<Rc<CHILD>>,
>;

fn get_obj<CHILD: WidgetHolder>(obj: &ChildrenHolder<CHILD>) -> Option<Rc<CHILD>> {
    obj.get()
}

pub trait OutletAdapter<CHILD: WidgetHolder> {
    type AdditionResult;
    type ParentData;

    fn children(&self) -> &[ChildrenHolder<CHILD>];

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
            .filter_map(get_obj as fn(&ChildrenHolder<CHILD>) -> Option<Rc<CHILD>>)
    }

    //todo fn remove_remnants(&mut self);

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<CHILD>;

    //todo: removing children
}


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
pub struct Outlet<CHILD, Parent>
where 
    CHILD: WidgetHolder + std::fmt::Debug + Child<Parent, CHILD>,
    Parent: NativeWidget + OutletAdapter<CHILD>
{
    ///Vector responsible for storing all the Children.
    /// 
    /// Uses a [`ChildrenHolder`] instead of the children directly
    pub(crate) children: Vec<ChildrenHolder<CHILD>>,

    _marker: std::marker::PhantomData<Parent>,
}
impl<CHILD, Parent>  Default for Outlet<CHILD, Parent>
where 
    CHILD: WidgetHolder + std::fmt::Debug + Child<Parent, CHILD>,
    Parent: NativeWidget + OutletAdapter<CHILD>
{
    fn default() -> Outlet<CHILD, Parent> {
        Self { 
            children: vec![],
            _marker: std::marker::PhantomData,
        }
    }
}
impl<CHILD, Parent> Outlet<CHILD, Parent>
where 
    CHILD: WidgetHolder + std::fmt::Debug + Child<Parent, CHILD>,
    Parent: NativeWidget + OutletAdapter<CHILD>
{
    /// Creates an Outlet with no children.
 

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

    pub(crate) fn children(&self) -> &[ChildrenHolder<CHILD>] {
        &self.children[0..self.children.len()]
    }

    pub(crate) fn add_child<T>(&mut self, child: T, parent: &Parent::ParentData) -> NativeResult<()>
    where
        T: Into<CHILD>,
    {
        let into_child = child.into();
        into_child.adding_to(parent);
        self.children
            .push(ChildrenHolder::Ours(Rc::new(into_child)));

        Ok(())
    }
}
