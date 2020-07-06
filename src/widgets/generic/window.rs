/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module contains a generic [`Window`] adapter and a structure
//! for generic parameters that work across all OS's.

use crate::data::ListenerType;
use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::WindowChildren;
use crate::widgets::{Child, MainMenuChildren, NativeWidget, OutletAdapter, RootChildren, System};

/// Generic parameters for creating and customizing Windows
///
/// All fields are optional and will either use OS Default values or sensible
/// custom default values where appropriate. Check Documentation of native Window Parameters for more details.
/// - [`CocoaWindowParameters`](crate::widget::cocoa::CocoaWindowParameters)
///
/// The above native window parameter struct implement the `From` trait to
/// get os specific parameters from this.
/// ```rust
/// use plating::widgets::generic::WindowParameters;
///
/// let params = WindowParameters::default();
///
/// #[cfg(target_os = "macos")]
/// let native: plating::widgets::cocoa::CocoaWindowParameters = params.into();
/// ```
///
/// You cannot generate a WindowParameter from a native Parameter struct, because they have more information that might be lost.
#[derive(Debug, Clone, Default, Serialize, Deserialize)] //not required but useful
#[derive(Eq, PartialEq)] //required in cached version
pub struct WindowParameters {
    /// Sets the Position and Size of the window
    pub rect: Option<crate::Rect>,
    /// Sets the title of the window
    pub title: Option<String>,

    /// Makes the window resizable.
    pub resizable: Option<bool>,
    /// Makes the window closable.
    pub closable: Option<bool>,
    /// Makes the window miniaturizable
    pub miniaturizable: Option<bool>,
    /// Makes the window maximizable.
    /// (Title and Main menu will still be shown)
    pub maximizable: Option<bool>,
    /// Allow the window to be fullscreen
    ///
    /// (Title and main menu will not be shown)
    pub fullscreenable: Option<bool>,
}

/* TODO: remove
/// Generic Window Adapter Widget
///
/// # Usage
/// - todo: creation example
///
/// - todo: apply example
///
/// - todo: add generic button
///
/// - todo: add native button
///
/// # Native Implementations
/// See the native implementations for more customization options (non cross-platform).
/// - [`CocoaWindow`](crate::widgets::cocoa::CocoaWindow)
#[derive(Debug)]
pub struct Window<S: System> {
    /// stores the underlying native widget.
    /// Most functions like `apply` are just forwarded to this.
    native: S::WindowType,
}
impl<S: System> Widget for Window<S> {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = WindowParameters;
}
impl<S: System> WidgetHolder for Window<S> {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl<S: System> GenericWidget<S> for Window<S> {
    type NativeParameterType = <S::WindowType as Widget>::PARAMS;
    type NativeType = S::WindowType;

    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self, S> {
        S::WindowType::new_with_name(name, settings)
            .map(|native| Window { native })
            .map_err(|native_error| native_error.into())
    }
}

impl<S: System> OutletAdapter<WindowChildren<S>, S> for Window<S> {
    type ErrorType = crate::error::PlatingError<S>;
    type ParentData = <S::WindowType as OutletAdapter<WindowChildren<S>, S>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<WindowChildren<S>>] {
        <S::WindowType as OutletAdapter<WindowChildren<S>, S>>::children(&self.native)
    }

    fn add_child<T>(&mut self, child: T) -> std::result::Result<(), Self::ErrorType>
    where
        T: Into<WindowChildren<S>>,
    {
        <S::WindowType as OutletAdapter<WindowChildren<S>, S>>::add_child(&mut self.native, child)
            //let child_into: WindowChildren = child.into();
            .map_err(|native_error| native_error.into())
    }
}*/

pub trait WindowHandlerTrait {
    fn set_resize_handler(&mut self, handler: Box<impl FnMut()>);
    fn add_resize_listener(&mut self, when: ListenerType, handler: Box<impl FnMut()>);
}

pub trait NativeWindow<S: System>:
    NativeWidget<S, PARAMS = S::WindowParameterType>
    + OutletAdapter<WindowChildren<S>, S>
    + OutletAdapter<MainMenuChildren<S>, S>
    + WindowHandlerTrait
    + Child<S::RootType, RootChildren<S>, S>
{
}

/*
impl<S> WindowHandlerTrait for Window<S>
where
    S: System,
{
    fn setResizeHandler(&mut self, mut handler: Box<impl FnMut()>) {
        self.native.setResizeHandler(Box::new(|| handler()));
    }

    fn addResizeListener(&mut self, when: ListenerType, mut handler: Box<impl FnMut()>) {
        self.native.addResizeListener(when, Box::new(|| handler()));
    }
}
*/
