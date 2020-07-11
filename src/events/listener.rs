/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListenerType {
    Before,
    After, //After listener get the HANDLED value of the handler (if any)
}
