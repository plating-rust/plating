/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::Named;
use crate::features::serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DropEvent {}

pub trait DropTopic {
    //only before, no way to cancel!
    fn add_listener(&self, handler: Box<impl FnMut(&DropEvent, &dyn Named)>);
}
