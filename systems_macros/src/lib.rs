/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![deny(
    //missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::missing_doc_code_examples,
    missing_debug_implementations,
    unused_import_braces,
    unused_crate_dependencies
)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(proc_macro_diagnostic)]

//! Macros used by the plating_systems crate.
//!
//! You can either use this crate directly if you want to build your
//! plating system from scratch.
//! Alternatively you can depend on the `plating_systems` crate which re-exports
//! these macros as well.
//!
//! ## Feature flags
//! None for this crate.
//!
//! ## Use via plating_systems
//! All 3 macros are also exported through plating_systems.
//! ```
//! // Instead of using this crate directly, prefer using it through plating_systems
//! pub use plating_systems::{build_widget_list, marker, tag};
//! ```
//!
//! ## Where are the macros created by 'build_widget_list'
//! They are in their own crate, but are exposed through 'plating_systems' as
//! well.

use proc_macro::TokenStream;

mod build_widget_list;
mod marker_impl;
mod tag;
mod utils;

/// Macro to create helper traits used internally by the systems logic.
/// These Helper traits need to be created for each widget type you want the
/// systems to support.
/// ```no_run
/// use plating_systems::systems::{System, SystemDefinition, SystemsList};
/// use plating_systems::tags::Tag;
/// use plating_systems_macros::marker;
///
/// pub trait TraitOfWidget {
///     //...
/// }
///
/// marker! {
///    pub type NameOfWidget: TraitOfWidget;
/// }
/// ```
///
/// Template Parameters are supported as well, so here is a more realistic
/// example. If your widgets need Template Parameters, you have to enable
/// '#![feature(generic_associated_types)]' however on a nightly rust
/// version.
/// ```no_run
/// # #![allow(incomplete_features)]
/// // Required for types with Template Parameters
/// #![feature(generic_associated_types)]
/// use plating_core::utils::outlet::OutletHolder;
/// use plating_core::widgets::Button;
/// use plating_systems::systems::{System, SystemDefinition, SystemsList};
/// use plating_systems::tags::Tag;
/// use plating_systems_macros::marker;
///
/// marker! {
///    pub type Button<OUTLET: OutletHolder>: Button<OUTLET>;
/// }
/// ```
/// the 'marker!' macro will create 3 Traits for the above invocation:
/// - `ButtonAvailable`: An empty trait used by 'SystemDefinition's to indicate
///   they support Buttons.
/// - `AllHaveButton`: An empty trait that has an automatic implementation for
///   'Tags's where all 'SystemDefinition's support Buttons (i.e. implement the
///   'ButtonAvailable' Trait).
/// - `HasTrait`: A trait that needs to be implemented for a System. Only has
///   one type Definition equal to the input type of the macro. In above example
///   this is ```pub type Button<OUTLET: OutletHolder>: Button<OUTLET>;```
///
/// NOTE: you do not need to create these helpers for plating internal widgets.
/// These are defined in the crate `plating_systems` in the `types` module.
#[proc_macro]
pub fn marker(input: TokenStream) -> TokenStream {
    marker_impl::marker(input)
}

/// Macro to create a Tag - A Tag represents a list of Systems.
///
/// Given a Trait and a list of Systems, this macro
/// - creates 'Tag<SYS>' where SYS is the provided Systems,
/// - updates the traits so that all 'types' defined on it,are only available
///   when all provided Systems support it.
///
/// ## Example 1
/// ```
/// use plating_systems::tags::Tag;
/// use plating_systems_macros::tag;
///
/// pub trait AdditionalWidgetTrait {}
///
/// #[tag(
///    for (
///        #[cfg(all(feature = "cocoa", target_os = "macos"))]
///        plating_systems::systems::Cocoa,
///        #[cfg(all(feature = "winui3", target_os = "windows"))]
///        plating_systems::systems::WinUI3,
///    )
/// )]
/// /// Our Custom Tag
/// pub trait CustomTag {
///     // OPTIONAL: You can still add you own types if you want.
///     type AdditionalWidget: AdditionalWidgetTrait;
/// }
/// ```
/// ## DRY
/// Don't repeat yourself, use 'plating_systems::default_widgets' if you want
/// your Tag to possibly enable all Widgets supported by plating.
///
/// ```
/// use plating_systems::default_widgets;
/// use plating_systems::tags::Tag;
/// use plating_systems_macros::tag;
///
/// pub trait AdditionalWidgetTrait {}
///
/// // this will automatically inject all types
/// #[default_widgets] //NEEDS TO BE ABOVE THE TAG MACRO
/// #[tag(
///    for (
///        #[cfg(all(feature = "cocoa", target_os = "macos"))]
///        plating_systems::systems::Cocoa,
///        #[cfg(all(feature = "winui3", target_os = "windows"))]
///        plating_systems::systems::WinUI3,
///    )
/// )]
/// /// Our Custom Tag
/// pub trait CustomTag {
///     // OPTIONAL: You can still add you own types if you want.
///     type AdditionalWidget: AdditionalWidgetTrait;
/// }
/// ```
///
/// As you can see, this cuts down a lot of boilerplate code by automatically
/// injecting all the widget types supported by plating. As you can see, you can
/// still add custom types if you want. ## Determine if all Systems provide a
/// type What can you actually do with this?
/// TODO: example of this
/// This is done by updating all 'type definitions'
/// from ```type Name: Type``` to ```type Name where {{TAG}}: AllHaveName, Self:
/// HasButton: Type```.
#[proc_macro_attribute]
pub fn tag(attr: TokenStream, item: TokenStream) -> TokenStream {
    tag::tag(attr, item) //how is there no naming conflict?!? :)
}

/// A lot of the systems logic is based on lists of types.
/// In order to automate and cut back on repeating all those types all the time,
/// `plating` comes with a `default_widgets` macro.
///
/// The default_widgets macro can be used like this:
/// ```
/// # #![allow(incomplete_features)]
/// #![feature(generic_associated_types)] // Required for default_widgets
///
/// use plating_systems::default_widgets;
/// # use plating_core::widgets::{Button, Window};
/// # use plating_core::utils::outlet::{ChildrenOutlet, MenuOutlet, Outlet, OutletHolder};
/// //This macro is auto generated by `build_widget_list`
/// #[default_widgets]
/// pub trait SomeTrait {}
/// ```
/// Writing this `default_widgets` macro requires some special handling and this
/// is where `build_widget_list` comes into play.
/// You have to provide the name of the macro to create
/// and a list of types (including template parameters).
/// ```ignore
/// use plating_systems_macros::build_widget_list;
/// //This generates a widgets_list macro that can be used the same way as
/// //The `default_widgets` macro that ships with plating
/// build_widget_list!{
///    widgets_list = {
///        type Window<OUTLET: OutletHolder
///                          + Outlet<MenuOutlet>
///                          + Outlet<ChildrenOutlet>>: Window<OUTLET>;
///        type Button<OUTLET: OutletHolder>: Button<OUTLET>;
///        // add as many widgets as you like
///        // ...
///    };
/// }
/// ```
/// When the types defined require Template parameters, the resulting macro
/// requires ```generic_associated_types```.
///
/// NOTE: since this defines a `proc_macro_attribute`, this macro requires the
/// following in `Cargo.toml`
/// ```toml
/// [lib]
/// proc-macro = true
/// ```
#[proc_macro]
pub fn build_widget_list(item: TokenStream) -> TokenStream {
    build_widget_list::build_widget_list(item)
}

///We use them in doc tests, does not stop
///rust to complain that we are not using the dev-dependencies
#[cfg(test)]
use plating_core as _;
#[cfg(test)]
use plating_systems as _;
