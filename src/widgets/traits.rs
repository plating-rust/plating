/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing traits used for all widgets.
//!
//! These are non-Platform specific and can safely be used
//! to write platform-independent code.

use crate::features::serde::Deserialize;
use crate::widgets::outlet::Outlet;
use crate::widgets::System;
use std::rc::{Rc, Weak};

/// Enum representing the EventState after a Event Callback was called.
///
/// # Example
/// todo: example callback return handled and unhandled on some condition
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum EventState {
    /// represent that the event was handled and no further event handlers should be called
    HANDLED,
    /// this event has not yet been handled.
    UNHANDLED,
}
/// Callback type definition.
///
/// Callback handlers must adhere to this type definition.
pub type Callback<T, W, E = ()> = dyn FnMut(&T, &mut W) -> Result<EventState, E>;

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

// todo: EQ implementation should check 'native' handle on backend as well as 'name'
/// Base Widget trait. (used by native and generic widgets)
///
/// ## Requirements
/// `Widgets` implementing this trait should also implement `std::fmt::Debug`
/// as well as [`WidgetHolder`].
///
/// ## See also
/// When you implement a widget you probably want to also implement one of the more specific
/// widget traits. See [`GenericWidget`] and [`NativeWidget`]
pub trait Widget
where
    Self: WidgetHolder + std::fmt::Debug,
{
    /// The Parameter type this struct requires when creating or applying changes to.
    type PARAMS;
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
/// `NativeWidget`s need the [`Widget`] trait.<br>
/// `NativeWidget`s need the `Sized` trait.
///
/// # Example
/// ## Implementation
/// A basic native widget implementation.
/// ```rust
/// use plating::widgets::{System, Widget, WidgetHolder, NativeWidget};
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
/// impl Widget for CocoaExampleWidget { //trait impl required by generic widget
///    // A struct containing parameters to customize a Widget.
///    // Empty struct in our case, but could be anything
///    type PARAMS = CocoaExampleParameters;
/// }
/// impl WidgetHolder for CocoaExampleWidget { //trait impl required by widget
///     // Returns the plating name (not a backend internal one)
///     // *NOTE*: no setter because the name should not change.
///     fn name(&self) -> &str {
///        &self.name.as_str()
///    }
/// }
/// impl NativeWidget<CocoaSystem> for CocoaExampleWidget {
///     fn new_with_name<T>(name: String, settings: T) -> CocoaResult<Self>
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
pub trait NativeWidget<S>
where
    Self: Widget + Sized,
    S: System,
{
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

    //todo: remove fn get_handler(&self) -> &Self::EventHandlerType;
    //todo: remove fn get_handler_mut(&mut self) -> &mut Self::EventHandlerType;
}

pub trait Child<ParentType, ChildType, S>
where
    ChildType: WidgetHolder,
    ParentType: NativeWidget<S> + Outlet<ChildType, S>,
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

trait LifecycleDelegate
where
    Self: Sized + std::fmt::Debug,
{
    fn created();

    fn before_display();
    fn displayed();

    fn before_destroy();
    fn destroyed();
}
