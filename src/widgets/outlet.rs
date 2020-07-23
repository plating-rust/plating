/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::utils::Identity;
use crate::widgets::{default_system, System};

pub trait Outlet<CHILD, S = default_system>
where
    CHILD: Identity,
    S: System + ?Sized,
{
    type ParentData;

    fn iter(&self) -> std::slice::Iter<'_, CHILD>;
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, CHILD>;

    fn get_by_id<STR: std::borrow::Borrow<str>>(&self, id: STR) -> Option<&CHILD> {
        self.iter().find(|obj: &&CHILD| obj.id() == id.borrow())
    }

    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn push_child<T>(&mut self, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<CHILD>;

    fn insert_child<T>(&mut self, index: usize, child: T) -> std::result::Result<(), anyhow::Error>
    where
        T: Into<CHILD>;

    fn remove_by_index(&mut self, index: usize) -> Option<CHILD>;
    fn remove_by_id<STR: std::borrow::Borrow<str>>(
        &mut self,
        id: STR,
    ) -> Result<CHILD, anyhow::Error>;
    fn remove_by_predicate<F: FnMut(&CHILD) -> bool>(
        &mut self,
        f: F,
    ) -> Result<CHILD, anyhow::Error>;
}
