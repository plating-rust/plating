/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! crate level documentation still missing

#![deny(
    //missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    //unstable_features,
    unused_qualifications,
)]
#![warn(
    //missing_debug_implementations,
    //missing_copy_implementations,
    unused_import_braces,
    //unused_crate_dependencies,
)]
//#![allow(incomplete_features)]
//#[allow(unstable_features)]
#![cfg_attr(feature = "template-specialization", feature(specialization))]


#[cfg(feature = "serde")]
extern crate serde;


pub mod backend;

/// native widget traits including
/// - platform specific attributes
/// - low level callbacks
pub mod native;

/// widget implementations as
/// well as platform independent traits:
/// - platform independent attributes
/// - events for each widget: unified behaviour for callbacks
pub mod widget;

/// Structural generators (Generate widget trees based on settings)
pub mod manager;

pub mod utils;

/// Lib version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type PlatingResult<T> = Result<T, anyhow::Error>;

pub mod prelude {
    pub use crate::backend::prelude::*;
    pub use crate::manager::prelude::*;
    pub use crate::native::prelude::*;
    pub use crate::widget::prelude::*;
    pub use crate::widget::WindowWidget;
}


#[cfg(test)]
mod tests {
    /*
    use crate::prelude::*;
    use crate::utils::children_list;
    use crate::widget::{AppOutlet, DefaultSystem, WindowOutlet};
    #[test]
    fn it_works() -> crate::PlatingResult<()> {
        let window: <DefaultSystem as System>::Window<(), _, _> =
            <DefaultSystem as System>::Window::new(
                (),
                WindowOutlet {
                    children: (),
                    menu:     (),
                },
            )?;
        let _app = <DefaultSystem as System>::App::new_with_state(
            (),
            (),
            AppOutlet {
                children: children_list!(window),
            },
        )?;


        /*
        let label = Label(String::from("yay"));
        let mut d = <DefaultSystem as System>::Window::<(), ()>::default();
        {
            use crate::native::cocoa::CocoaWindow;
        }
        d.apply((label, ())).unwrap();

        let d = <DefaultSystem as System>::Window::<(u8, u8), _>::new_with_state((5, 5), (), ());
        //should not compile

        //let e = WindowWidget::<()>::new(&());*/

        Ok(())
    }*/
}




// todo: move to sth more appropriate: + implement :)

pub trait ButtonType<T> {
    fn new(t: T) -> Self;
}
pub struct CocoaButton<T> {
    _t: T,
}
impl<T> ButtonType<T> for CocoaButton<T> {
    fn new(t: T) -> Self {
        Self {_t: t}
    }
}
#[cfg(target_os = "windows")]
pub struct WinUI3Button<T> {
    t: T,
}
#[cfg(target_os = "windows")]
impl<T> ButtonType<T> for WinUI3Button<T> {
    fn new(t: T) -> Self {
        Self {t}
    }
}

pub trait TabsType<T> {
    fn new(t: T) -> Self;
}
pub struct TabsNotImplemented {
}
impl<T> TabsType<T> for TabsNotImplemented {
    fn new(_t: T) -> Self { unimplemented!() }
}
#[cfg(target_os = "macos")]
pub struct CocoaTabs<T> {
    _t: T,
}
impl<T> TabsType<T> for CocoaTabs<T> {
    fn new(t: T) -> Self {
        Self {_t: t}
    }
}
