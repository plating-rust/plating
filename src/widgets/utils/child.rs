/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::outlet::Outlet;
use crate::widgets::utils::Named;
use crate::widgets::{System, Widget};

pub trait Child<ParentType, ChildType, S>
where
    ChildType: Named,
    ParentType: Widget<S> + Outlet<ChildType, S>,
    S: System,
{
    fn adding_to(&self, _parent: &ParentType::ParentData) {}
}
