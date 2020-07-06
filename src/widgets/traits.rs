/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing traits used for all widgets.
//!
//! These are non-Platform specific and can safely be used
//! to write platform-independent code.

use crate::features::serde::Deserialize;
use crate::widgets::events::LifecycleHandler;
use crate::widgets::outlet::Outlet;
use crate::widgets::System;
use std::rc::{Rc, Weak};

/// Very basic trait implemented bothy by widgets themselves and
/// any kind of `Pointer` or other Widget indirection.
///
/// # Requirements
/// When implementing this trait, make sure that `name()`always returns the same value
/// and does not change during the lifetime of this instance.
pub trait WidgetHolder {
    /// Get the name of this widget or the widget this object is pointing to.
    fn name(&self) -> &str;
}

/// Trait for all Native Widget Objects.
///
/// `NativeWidgets` have the following responsibilities:
/// - Create the widget on the backend in their constructors
/// - Provide a way to apply and change settings.
///
/// All Widgets in the following modules implement this trait
/// - [`plating::widgets::cocoa`](crate::widgets::cocoa)
/// - [`plating::widgets::win`](crate::widgets::win)
/// - [`plating::widgets::mock`](crate::widgets::mock)
///
/// # Requirements
/// `NativeWidget`s needs to implement the [`Widget`] trait.<br>
/// `NativeWidget`s need the `Sized` trait.<br>
/// `NativeWidget`s need to implement the [`WidgetHolder`] trait.
/// `NativeWidget`s need to implement the `std::fmt::Debug` trait.
///
/// # Example
/// ## Implementation
/// A basic native widget implementation.
/// ```rust
/// use plating::widgets::{System, Widget, WidgetHolder};
/// use plating::widgets::events::{ListenerType, LifecycleHandler};
/// use plating::widgets::cocoa::{CocoaSystem, CocoaDefaultHandleType};
/// use plating::widgets::cocoa::error::{CocoaError, CocoaResult};
///
/// // Some imaginary config for our widget
/// struct CocoaExampleParameters {
///    width: u32,
///    height: u32
/// }
///
/// #[derive(Debug)]
/// struct CocoaExampleWidget {
///     // Native Widgets themselves should not hold state
///     // unless necessary for their work. That's why a lot of
///     // widgets don't hold much more state than the name
///     name: String,
///
///     handle: CocoaDefaultHandleType,
/// }
///
/// impl WidgetHolder for CocoaExampleWidget { //trait impl required by widget
///     // Returns the plating name (not a backend internal one)
///     // *NOTE*: no setter because the name should not change.
///     fn name(&self) -> &str {
///        &self.name.as_str()
///    }
/// }
/// impl Widget<CocoaSystem> for CocoaExampleWidget {
///    // A struct containing parameters to customize a Widget.
///    // Empty struct in our case, but could be anything
///    type PARAMS = CocoaExampleParameters;
///
///    fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
///    where
///        T: Into<Self::PARAMS> {
///        let mut result = Self {name, handle: todo!() };
///        result.apply(settings);
///        Ok(result)
///    }
///     fn apply<T>(&mut self, settings: T) -> Result<(), CocoaError>
///    where
///        T: Into<Self::PARAMS> {
///        todo!() //apply settings on the backend
///    }
///
///     fn native(&self) -> &<CocoaSystem as System>::InternalHandle {
///        &self.handle
///     }
///     unsafe fn native_mut(&mut self) -> &mut <CocoaSystem as System>::InternalHandle {
///         &mut self.handle
///     }
/// }
///
/// impl LifecycleHandler for CocoaExampleWidget {
///     fn add_create_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
///         todo!()
///     }
///
///     fn add_display_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
///         todo!()
///     }
///
///     fn add_destroy_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
///         todo!()
///     }
///
///     fn add_apply_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
///         todo!()
///     }
/// }
/// ```
///
/// Now that's a lot of boilerplate for a simple widget.
/// That's why there is a macro.
/// todo: macro example
/// ## Usage
///
/// # Error Handling
/// Functions in this trait, that can fail, return a `NativeResult<Self>`.
/// If the called need a `PlatingResult<Self>`, you can use `from`/`into`
///
pub trait Widget<S>
where
    Self: WidgetHolder + std::fmt::Debug + Sized + LifecycleHandler,
    S: System,
{
    /// The Parameter type this struct requires when creating or applying changes to it.
    type PARAMS;

    fn new<T>(settings: T) -> Result<Self, S::ErrorType>
    where
        T: Into<Self::PARAMS>,
    {
        Self::new_with_name(uuid::Uuid::new_v4().to_string(), settings)
    }
    fn new_with_name<T>(name: String, settings: T) -> Result<Self, S::ErrorType>
    where
        T: Into<Self::PARAMS>;

    fn apply<T>(&mut self, settings: T) -> Result<(), S::ErrorType>
    where
        T: Into<Self::PARAMS>;

    fn native(&self) -> &S::InternalHandle;
    unsafe fn native_mut(&mut self) -> &mut S::InternalHandle;
}

pub trait Child<ParentType, ChildType, S>
where
    ChildType: WidgetHolder,
    ParentType: Widget<S> + Outlet<ChildType, S>,
    S: System,
{
    fn adding_to(&self, _parent: &ParentType::ParentData) {}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub enum ChildrenHolder<T: ?Sized + WidgetHolder> {
    #[serde(skip)]
    Weak(Weak<T>),
    Ours(Rc<T>),
}

#[cfg(feature = "serde")]
impl<T: WidgetHolder + serde::Serialize> serde::Serialize for ChildrenHolder<T> {
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

impl<T: WidgetHolder> ChildrenHolder<T> {
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
                *self = ChildrenHolder::Weak(w);
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
impl<T: ?Sized + PartialEq + WidgetHolder> PartialEq for ChildrenHolder<T> {
    fn eq(&self, other: &ChildrenHolder<T>) -> bool {
        match (self, other) {
            (Self::Weak(lhs), Self::Weak(rhs)) => lhs.ptr_eq(rhs),
            (Self::Ours(lhs), Self::Ours(rhs)) => Rc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}

impl<T: WidgetHolder> std::fmt::Pointer for ChildrenHolder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weak(w) => std::fmt::Pointer::fmt(&w, f),
            Self::Ours(o) => std::fmt::Pointer::fmt(&o, f),
        }
    }
}
