/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub mod cocoa;

pub trait Backend: std::fmt::Debug + Sized {
    /*type Window<T, C: ChildrenList, M: ChildrenList>: NativeWidget<T, System = Self>
        + WindowWidget<T, Self>;
    type App<T, C: ChildrenList>: NativeWidget<T, System = Self> + AppWidget<T, C, Self>;

    /// Returns the name of this system.
    fn name() -> &'static str;*/
}


/*#[cfg(target_os = "macos")]
pub type DefaultSystem = cocoa::Cocoa;*/

pub mod prelude {
    pub use super::Backend;
}
