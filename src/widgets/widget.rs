/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::events::LifecycleHandler;
use crate::widgets::utils::Named;
use crate::widgets::System;

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
/// use plating::widgets::{System, Widget};
/// use plating::widgets::utils::{Named};
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
/// impl Named for CocoaExampleWidget { //trait impl required by widget
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
    Self: Named + std::fmt::Debug + Sized + LifecycleHandler,
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