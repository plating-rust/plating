/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module contains a generic [`Window`] adapter and a structure
//! for generic parameters that work across all OS's.

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::native::{NativeWindow, NativeWindowParameters};
use crate::widgets::WindowChildren;
use crate::widgets::{
    OutletAdapter, ChildrenHolder, GenericWidget, NativeWidget, Widget, WidgetHolder,
};
use crate::PlatingResult;

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
pub struct Window {
    /// stores the underlying native widget.
    /// Most functions like `apply` are just forwarded to this.
    native: NativeWindow,
}
impl Widget for Window {
    /// Means that `new_...` and `apply` functions require [`WindowParameters`]
    type PARAMS = WindowParameters;
}
impl WidgetHolder for Window {
    fn name(&self) -> &str {
        &self.native.name()
    }
}
impl GenericWidget for Window {
    type NativeType = NativeWindow;
    type NativeParameterType = NativeWindowParameters;
    /// does this show up?
    fn native(&self) -> &Self::NativeType {
        &self.native
    }
    fn native_mut(&mut self) -> &mut Self::NativeType {
        &mut self.native
    }
    fn new_with_name(name: String, settings: Self::PARAMS) -> PlatingResult<Self> {
        NativeWindow::new_with_name(name, settings)
            .map(|native| Window { native })
            .map_err(|native_error| native_error.into())
    }
}

impl OutletAdapter<WindowChildren> for Window {
    type AdditionResult = PlatingResult<()>;
    type ParentData = <NativeWindow as OutletAdapter<WindowChildren>>::ParentData;

    fn children(&self) -> &[ChildrenHolder<WindowChildren>] {
        self.native.children()
    }

    fn add_child<T>(&mut self, child: T) -> Self::AdditionResult
    where
        T: Into<WindowChildren>,
    {
        //let child_into: WindowChildren = child.into();
        self.native
            .add_child(child)
            .map_err(|native_error| native_error.into())
    }
}
