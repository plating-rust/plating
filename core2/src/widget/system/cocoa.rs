/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![cfg(target_os = "macos")]

use crate::prelude::Backend;
//use crate::utils::ChildrenList;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cocoa {}


impl Backend for Cocoa {
    /*
    type App<T, C: ChildrenList> = crate::native::cocoa::CocoaAppWidget<T, C>;
    type Window<T, C: ChildrenList, M: ChildrenList> =
        crate::native::cocoa::CocoaWindowWidget<T, C, M>;

    fn name() -> &'static str {
        "cocoa"
    }*/
}
