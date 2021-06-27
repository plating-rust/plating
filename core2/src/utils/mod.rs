/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

mod serde;
pub(crate) use crate::utils::serde::{
    Deserialize,
    DeserializeEx,
    DeserializeTrait,
    Serialize,
    SerializeTrait,
};

pub mod data;

mod properties;
pub use properties::{setting_list, Property, SettingsList};

mod children;
pub use children::{
    children_list,
    ChildOf,
    ChildrenList,
    ChildrenOutlet,
    MenuOutlet,
    Outlet,
    OutletHolder,
};
