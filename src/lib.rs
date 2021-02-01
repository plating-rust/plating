// Copyright 2021 Marcel Lambert.
//
// See LICENSE for licensing information.

//! crate level documentation still missing

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![warn(unused_crate_dependencies)]
// we don't want to keep unused dependencies around, but cargo_husky is needed
// for git hooks even though we don't really use it in our codebase.
// hence we `fake` use it
#[cfg(test)]
use cargo_husky as _;



mod backend;

/// native widget traits including
/// - plattform specific attributes
/// - low level callbacks
mod native;

/// widget implementations as
/// well as platform independent traits:
/// - plattform independant attributes
/// - events for each widget: unified behaviour for callbacks
mod widget;

/// Structural generators (Generate widget trees based on settings)
mod manager;

mod utils {
    mod data {
        //! Types like Color, Width, LocalizedString,
    }

    mod attr {
        mod getters {}
        mod setters {}
        mod pusher {}
    }
}

/// Level 0 abstractions
pub mod lvl0 {
    pub use super::backend::*;
}
/// Level 1 abstractions
pub mod lvl0_5 {
    pub use super::native::*;
}
/// Level 2 abstractions
pub mod lvl1 {
    pub use super::widget::*;
}
/// Level 3 abstractions
pub mod lvl2 {
    pub use super::manager::*;
}

/// Lib version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
