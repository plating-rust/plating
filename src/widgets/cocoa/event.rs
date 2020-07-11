/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::events::Event;
use cocoa::appkit::{NSEvent, NSEventModifierFlags};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CocoaEvent<T>
where
    T: NSEvent + Copy + Sized,
{
    native_event: T,
}

impl<'a, T> Event<'a> for CocoaEvent<T>
where
    T: NSEvent + Copy + Sized,
{
    fn timestamp(&self) -> u64 {
        unsafe { (&self.native_event.timestamp() * 1000.0) as u64 }
    }
    fn target(&self) -> &'a dyn crate::prelude::Named {
        //todo: use hash lookup table
        todo!()
    }
}

impl<T> CocoaEvent<T>
where
    T: NSEvent + Copy + Sized,
{
    pub fn native(&self) -> &T {
        &self.native_event
    }
}

impl<T> From<T> for CocoaEvent<T>
where
    T: NSEvent + Copy + Sized,
{
    fn from(event: T) -> Self {
        Self {
            native_event: event,
        }
    }
}

bitflags! {
    pub struct CocoaKeyModifiers: u64 {
        const SHIFT = NSEventModifierFlags::NSShiftKeyMask.bits();
        const CONTROL = NSEventModifierFlags::NSControlKeyMask.bits();
        const OPTION = NSEventModifierFlags::NSAlternateKeyMask.bits();
        const COMMAND = NSEventModifierFlags::NSCommandKeyMask.bits();
        const FUNTION = NSEventModifierFlags::NSFunctionKeyMask.bits();
    }
}
