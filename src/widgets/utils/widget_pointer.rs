/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::Deserialize;
use crate::widgets::utils::Named;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub enum WidgetPointer<T: ?Sized + Named> {
    #[serde(skip)]
    Weak(Weak<T>),
    Ours(Rc<T>),
}

impl<T: Named> WidgetPointer<T> {
    pub fn new(value: T) -> Self {
        Self::Ours(Rc::new(value))
    }

    #[must_use]
    pub fn get(&self) -> Option<Rc<T>> {
        match self {
            Self::Weak(w) => w.upgrade(),
            Self::Ours(o) => Some(o.clone()),
        }
    }

    #[must_use = "The parent object has downgraded the pointer to a weak one. If you do not use the result, the child is automatically removed."]
    pub fn require(&mut self) -> Option<Rc<T>> {
        match self {
            Self::Weak(w) => w.upgrade(),
            Self::Ours(o) => {
                let result = o.clone(); //make sure we have a strong pointer, otherwise we might drop the object on the next line
                let w = Rc::downgrade(o);
                *self = WidgetPointer::Weak(w);
                Some(result)
            }
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Weak(_) => None,
            Self::Ours(obj) => Some(obj.as_ref()),
        }
    }
}

impl<T: ?Sized + PartialEq + Named> PartialEq for WidgetPointer<T> {
    fn eq(&self, other: &WidgetPointer<T>) -> bool {
        match (self, other) {
            (Self::Weak(lhs), Self::Weak(rhs)) => lhs.ptr_eq(rhs),
            (Self::Ours(lhs), Self::Ours(rhs)) => Rc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}

impl<T: Named> std::fmt::Pointer for WidgetPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weak(w) => std::fmt::Pointer::fmt(&w, f),
            Self::Ours(o) => std::fmt::Pointer::fmt(&o, f),
        }
    }
}

#[cfg(feature = "serde")]
impl<T: Named + serde::Serialize> serde::Serialize for WidgetPointer<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.get() {
            Some(pointer) => pointer.as_ref().serialize(serializer),
            None => Err(serde::ser::Error::custom(
                "WidgetHolder contains weak reference to already deleted widget",
            )),
        }
    }
}
