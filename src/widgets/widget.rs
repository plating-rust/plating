/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */
use crate::widgets::utils::Named;
use crate::widgets::System;

/// Trait for all Widget Objects.
///
/// `Widgets` have the following responsibilities:
/// - Create the widget on the backend in their constructors
/// - Provide a way to apply and change settings.
///
/// All Widgets in the following modules implement this trait
/// - [`plating::widgets::cocoa`](crate::widgets::cocoa)
/// - [`plating::widgets::win`](crate::widgets::win)
/// - [`plating::widgets::mock`](crate::widgets::mock)
///
/// # Requirements
/// `Widget`s needs to implement the [`Widget`] trait.<br>
/// `Widget`s need the `Sized` trait.<br>
/// `Widget`s need to implement the [`Named`] trait.
/// `Widget`s need to implement the `std::fmt::Debug` trait.
///
/// # Example
/// ## Implementation
/// A basic native widget implementation.
/// ```rust
/// use plating::{PlatingResult};
/// use plating::widgets::{System, Widget};
/// use plating::widgets::utils::{Named};
/// use plating::events::{ListenerType};
/// use plating::widgets::cocoa::{CocoaSystem, CocoaDefaultHandleType};
/// use plating::widgets::cocoa::error::{CocoaError};
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
///    fn new_with_name<T>(name: String, settings: T) -> PlatingResult<Self>
///    where
///        T: Into<Self::PARAMS> {
///        let mut result = Self {name, handle: todo!() };
///        result.apply(settings);
///        Ok(result)
///    }
///     fn apply<T>(&mut self, settings: T) -> PlatingResult<()>
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
/// ```
///
/// Now that's a lot of boilerplate for a simple widget.
/// That's why there is a macro.
/// todo: macro example
/// ## Usage
///
/// # Error Handling
/// Functions in this trait, that can fail, return a `Result<Self, anyhow::Error>`.
///
pub trait Widget<S>
where
    Self: Named + std::fmt::Debug + Sized,
    S: System,
{
    /// The Parameter type this struct requires when creating or applying changes to it.
    type PARAMS;

    /// Constructor that takes settings and returns Self.
    ///
    /// The constructor can fail if the settings have problems.
    ///
    /// # Additional
    /// Widgets are encouraged to implement the [`Default`] trait when appropriate.
    fn new<T>(settings: T) -> Result<Self, anyhow::Error>
    where
        T: Into<Self::PARAMS>,
    {
        Self::new_with_name(uuid::Uuid::new_v4().to_string(), settings)
    }
    fn new_with_name<T>(name: String, settings: T) -> Result<Self, anyhow::Error>
    where
        T: Into<Self::PARAMS>;

    fn apply<T>(&mut self, settings: T) -> Result<(), anyhow::Error>
    where
        T: Into<Self::PARAMS>;

    //todo: move out of obvious api so we prevent accidental non cross-plattformness
    fn native(&self) -> &S::InternalHandle;
    unsafe fn native_mut(&mut self) -> &mut S::InternalHandle;
}
