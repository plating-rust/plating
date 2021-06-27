/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

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
//#![cfg_attr(feature = "template-specialization", feature(specialization))]

#[cfg(feature = "serde")]
extern crate serde;

/// A private struct. Used to make sure that
/// some structs cannot be build without using the appropriate constructors.
pub(crate) struct Private {}

/// Lib version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default Result type used in plating
pub type PlatingResult<T> = Result<T, anyhow::Error>;

pub mod data;
pub mod utils;
pub mod widgets;

pub mod mock;

pub mod prelude {
    pub use super::data::prelude::*;
    pub use super::utils::prelude::*;
    pub use super::widgets::prelude::*;
}

#[cfg(test)]
mod tests {
    pub use super::prelude::*;
    pub use super::*;
    use crate::mock::{MockButtonOutlet, MockButtonWidget, MockWindowOutlet, MockWindowWidget};
    use crate::utils::children::children_list;
    use crate::widgets::WindowOutlet;

    #[test]
    fn it_works() -> PlatingResult<()> {
        let b = MockButtonWidget::new(MockButtonOutlet {})?;

        <MockWindowWidget<_> as mock::MockWindow<_>>::new(MockWindowOutlet {
            children: children_list!(b),
            menu:     (),
        })?;


        let _a = MockWindowWidget::new(WindowOutlet {
            children: (),
            menu:     (),
        })?;

        Ok(())
    }
}
