/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing struct of the same Name [`Outlet`].

use thiserror::Error;

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::outlet::Outlet;
use crate::widgets::utils::{Child, Connectable, Identity};
use crate::widgets::{System, Widget};

#[derive(Error, Debug)]
#[error("WidgetNotFound: {msg}")]
pub struct WidgetNotFound {
    msg: String,
    #[source] // optional if field name is `source`
    source: Option<anyhow::Error>,
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
/// # Assumptions
/// Plating assumes
/// - widget names are constant.
/// - widget names are unique. Not enforced by plating, users responsibility.
/// # Template Parameters
/// - `CHILD`: the kind of elements this Outlet can store.<br>
///     Usually realized by enums. See [`WindowChildren`](crate::widgets::WindowChildren) for an example.
///     <br><br>**Requirements**: Needs to implement [`WidgetHolder`] + `std::fmt::Debug`
///
/// # Drop
/// makes sure all elements receive the `disconnected` and `remove_from_parent` signals.
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    ///Vector responsible for storing all the Children.
    ///
    /// Uses a [`ChildrenHolder`] instead of the children directly
    children: Vec<CHILD>,

    ///Indicates that children in this outlet are connected to root
    connected: bool,

    _marker: std::marker::PhantomData<Parent>,
    _marker2: std::marker::PhantomData<S>,
}
impl<CHILD, Parent, S> OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    fn prepare_child<T>(&self, child: T, parent: &Parent::ParentData) -> CHILD
    where
        T: Into<CHILD>,
    {
        let mut into_child = child.into();
        into_child.adding_to_parent(parent);
        if self.connected {
            into_child.connecting();
        }
        into_child
    }
}

impl<CHILD, Parent, S> Connectable for OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    /// Marks itself as connected and calls `fn connecting()` on all child elements
    ///
    /// # Preconditions
    /// Panics if already connected.
    fn connecting(&mut self) {
        if self.connected {
            panic!("Outlet Holder already connected")
        }
        for child in &mut self.children {
            child.connecting();
        }
        self.connected = true;
    }

    /// Marks itself as disconnected and calls `fn disconnecting()` on all child elements
    ///
    /// # Preconditions
    /// Panics if already disconnected.
    fn disconnecting(&mut self) {
        if !self.connected {
            panic!("Outlet Holder not yet connected")
        }
        for child in &mut self.children {
            child.disconnecting();
        }
        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}

impl<CHILD, Parent, S> Default for OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    /// Returns an empty, unconnected OutletHolder
    fn default() -> OutletHolder<CHILD, Parent, S> {
        Self {
            children: vec![],
            connected: false,
            _marker: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
        }
    }
}
impl<CHILD, Parent, S> OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    /// Returns an Iterator to the internal vec holding the children.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CHILD> {
        self.children.iter()
    }

    /// Returns a mut Iterator to the internal vec holding the children.
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, CHILD> {
        self.children.iter_mut()
    }

    /// Returns the capacity of the internal vector.
    ///
    /// See [Vec::capacity]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.children.capacity()
    }
    /// Reserves space for the specified amount of *additional* children.
    ///
    /// See [Vec::reserve]
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.children.reserve(additional)
    }
    /// Reserves space for the specified amount of children *in total*.
    ///
    /// See [Vec::reserve_exact]
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.children.reserve_exact(additional)
    }
    /// Resizes the vector to fit the current amount of children.
    ///
    /// See [Vec::shrink_to_fit]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
    }

    /// Empties the internal vector and makes sure children are `disconnected` and `removed from parent`.
    #[inline]
    pub fn clear(&mut self) {
        if self.connected {
            for child in &mut self.children {
                child.disconnecting();
            }
        }
        for child in &mut self.children {
            child.removing_from_parent();
        }
        self.children.clear()
    }

    /// Returns the number of children
    #[inline]
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns true if there are no children, false otherwise
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Inserts a child into this holder at the specified index.
    ///
    /// # Responsibilities
    /// Makes sure [`adding_to_parent`](crate::widgets::utils::Child::adding_to_parent) is called.<br>
    /// If this holder is [`connected`](OutletHolder::connected) will also call [`connecting`](crate::widgets::utils::connectable::Connectable::connecting) on the added child.
    ///
    /// See also [`push_child`](OutletHolder::push_child)
    pub fn insert_child<T>(
        &mut self,
        index: usize,
        child: T,
        parent: &Parent::ParentData,
    ) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<CHILD>,
    {
        self.children
            .insert(index, self.prepare_child(child, parent));

        Ok(())
    }

    /// Pushes a child at the and of this holder.
    ///
    /// # Responsibilities
    /// Makes sure [`adding_to_parent`](crate::widgets::utils::Child::adding_to_parent) is called.<br>
    /// If this holder is [`connected`](OutletHolder::connected) will also call [`connecting`](crate::widgets::utils::connectable::Connectable::connecting) on the added child.
    ///
    /// See also [`insert_child`](OutletHolder::insert_child)
    pub fn push_child<T>(
        &mut self,
        child: T,
        parent: &Parent::ParentData,
    ) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<CHILD>,
    {
        self.children.push(self.prepare_child(child, parent));

        Ok(())
    }

    /// Removes the child at the given index.
    ///
    /// # Responsibilities
    /// If this holder is [`connected`](OutletHolder::connected) will also call [`disconnecting`](crate::widgets::utils::connectable::Connectable::disconnecting) on the removed child.<br>
    /// Makes sure [`removing_from_parent`](crate::widgets::utils::Child::removing_from_parent) is called.
    ///
    /// See also [`remove_by_id`](OutletHolder::remove_by_id) and [`remove_by_predicate`](OutletHolder::remove_by_predicate).
    pub fn remove_by_index(&mut self, index: usize) -> CHILD {
        let child = &mut self.children[index];
        if child.connected() {
            child.disconnecting();
        }
        child.removing_from_parent();
        self.children.remove(index)
    }
    /// Removes the child with given id.
    ///
    /// # Responsibilities
    /// If this holder is [`connected`](OutletHolder::connected) will also call [`disconnecting`](crate::widgets::utils::connectable::Connectable::disconnecting) on the removed child.<br>
    /// Makes sure [`removing_from_parent`](crate::widgets::utils::Child::removing_from_parent) is called.
    ///
    /// NOTE: only removes the first child with the given id.
    ///
    /// See also [`remove_by_index`](OutletHolder::remove_by_index) and [`remove_by_predicate`](OutletHolder::remove_by_predicate).
    pub fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<CHILD, anyhow::Error> {
        self.remove_by_predicate(|obj: &CHILD| obj.id() == id.borrow())
            .map_err(|orig_error| {
                WidgetNotFound {
                    msg: format!("by name: {}", id.borrow()),
                    source: Some(orig_error),
                }
                .into()
            })
    }

    /// Removes the first child the given predicate evaluates to true.
    ///
    /// # Responsibilities
    /// If this holder is [`connected`](OutletHolder::connected) will also call [`disconnecting`](crate::widgets::utils::connectable::Connectable::disconnecting) on the removed child.<br>
    /// Makes sure [`removing_from_parent`](crate::widgets::utils::Child::removing_from_parent) is called.
    ///
    /// See also [`remove_by_index`](OutletHolder::remove_by_index) and [`remove_by_id`](OutletHolder::remove_by_id).
    pub fn remove_by_predicate<F: FnMut(&CHILD) -> bool>(
        &mut self,
        f: F,
    ) -> Result<CHILD, anyhow::Error> {
        match self.children.iter().position(f) {
            Some(pos) => Ok(self.remove_by_index(pos)),
            None => Err(WidgetNotFound {
                msg: String::from("by predicate"),
                source: None,
            }
            .into()),
        }
    }
}

impl<CHILD, Parent, S> Drop for OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    fn drop(&mut self) {
        self.clear()
    }
}

impl<CHILD, Parent, S> std::ops::Index<usize> for OutletHolder<CHILD, Parent, S>
where
    CHILD: Identity + std::fmt::Debug + Child<Parent, CHILD, S>,
    Parent: Widget<S> + Outlet<CHILD, S>,
    S: System,
{
    type Output = CHILD;

    fn index(&self, index: usize) -> &CHILD {
        &self.children[index]
    }
}
