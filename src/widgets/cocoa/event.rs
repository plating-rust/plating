/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

 use cocoa::appkit::NSEvent;

#[derive(Debug)]
pub struct CocoaEvent<T>
where
    T: NSEvent + Sized
{
    native_event: T,
}

impl<T> CocoaEvent<T>
where
    T: NSEvent + Sized
{
    pub fn native(&self) -> &T { 
        &self.native_event
    }
}