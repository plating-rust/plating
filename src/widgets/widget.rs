/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */
use crate::widgets::platform_dependant::NativeWidget;
use crate::widgets::utils::Identity;
use crate::widgets::System;
use std::borrow::Borrow;

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
/// NOTE: the name is expected to stay the same, so can only be set in the constructor.
/// Constructor Versions that don't take a name generate a [`Uuid`](uuid::Uuid::new_v4).
/// # Example
/// ## Implementation
/// A basic native widget implementation.
/// ```rust
/// use plating::{PlatingResult};
/// use plating::widgets::{System, Widget};
/// use plating::widgets::utils::{Identity};
/// use plating::widgets::platform_dependant::NativeWidget;
/// use plating::events::{ListenerType};
/// use plating::widgets::cocoa::{CocoaSystem, CocoaDefaultHandleType};
/// use plating::widgets::cocoa::error::{CocoaError};
///
/// use std::borrow::Borrow;
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
///     id: String,
///
///     handle: CocoaDefaultHandleType,
/// }
///
/// impl Identity for CocoaExampleWidget { //trait impl required by widget
///     // Returns the plating name (not a backend internal one)
///     // *NOTE*: no setter because the name should not change.
///     fn id(&self) -> &str {
///        &self.id.as_str()
///    }
/// }
/// impl Widget<CocoaSystem> for CocoaExampleWidget {
///    // A struct containing parameters to customize a Widget.
///    // Empty struct in our case, but could be anything
///    type PARAMS = CocoaExampleParameters;
///
///    fn new_with_id<STR, PARAMS>(id: STR, settings: PARAMS) -> PlatingResult<Self>
///    where
///        STR: Into<String>,
///        PARAMS: Borrow<Self::PARAMS>,
///    {
///        let mut result = Self {id: id.into(), handle: todo!() };
///        result.apply(settings);
///        Ok(result)
///    }
///    fn apply<PARAMS>(&mut self, settings: PARAMS) -> PlatingResult<()>
///    where
///        PARAMS: Borrow<Self::PARAMS>,
///    {
///        let settings = settings.borrow();
///        todo!() //apply settings on the backend
///    }
/// }
/// impl NativeWidget<CocoaSystem> for CocoaExampleWidget {
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
    Self: Identity + std::fmt::Debug + Sized + NativeWidget<S>,
    S: System + ?Sized,
{
    /// The Parameter type this struct requires when creating or applying changes to it.
    type PARAMS;

    /// Constructor that takes settings and returns Self.
    /// Generates a [`Uuid`](uuid::Uuid::new_v4) as the ID.
    ///
    /// The constructor can fail if the settings have problems.
    ///
    /// # Additional
    /// Widgets are encouraged to implement the [`Default`] trait when appropriate.
    ///
    /// See also: [`new_with_id`](Widget::new_with_id).
    fn new<PARAMS>(settings: PARAMS) -> Result<Self, anyhow::Error>
    where
        PARAMS: Borrow<Self::PARAMS>,
    {
        Self::new_with_id::<String, PARAMS>(uuid::Uuid::new_v4().to_string(), settings)
    }

    /// Constructor that takes an ID and settings and returns Self.
    ///
    /// See also: [`new`](Widget::new).
    fn new_with_id<STR, PARAMS>(id: STR, settings: PARAMS) -> Result<Self, anyhow::Error>
    where
        STR: Into<String>,
        PARAMS: Borrow<Self::PARAMS>;

    /// Applies settings to this widget
    ///
    /// See also: [`new`](Widget::new).
    fn apply<PARAMS>(&mut self, settings: PARAMS) -> Result<(), anyhow::Error>
    where
        PARAMS: Borrow<Self::PARAMS>;
}
