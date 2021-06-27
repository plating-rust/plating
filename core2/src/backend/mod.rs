/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![allow(unsafe_code)]

pub trait AsBackend<BACKEND> {
    fn as_backend(&self) -> &BACKEND;
    fn as_mut_backend(&mut self) -> &mut BACKEND;
}

pub mod cocoa;

pub mod prelude {}
