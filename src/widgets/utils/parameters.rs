/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::clone::Clone;
use std::cmp::PartialEq;
use std::default::Default;
use std::fmt::Debug;

/// Base trait for all Parameter structures.
///
/// # Requirements
/// To make dealing with parameters as consistent as possible,
/// struct implementing this trait also need to implement the following traits:
/// - [`std::fmt::Debug`]
/// - [`std::clone::Clone`]
/// - [`std::default::Default`]
/// - [`std::cmp::PartialEq`]
/// - [`std::hash::Hash`]
//todo: serialize/deserialize
pub trait Parameters: Debug + Clone + Default + PartialEq {
    /// All values in self that are not set, will be taken from rhs.
    /// todo: explain better
    fn merge(&mut self, rhs: Self) -> Result<(), anyhow::Error>;

    ///all values set in rhs will be set in self.
    /// todo: explain better
    fn on_top(&mut self, rhs: Self) -> Result<(), anyhow::Error>;
}
