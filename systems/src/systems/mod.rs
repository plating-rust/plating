/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

mod mock;
#[cfg(any(feature = "mock", doc))]
pub use mock::*;

mod android;
#[cfg(any(feature = "android", doc))]
pub use android::*;

mod ios;
#[cfg(any(feature = "ios", doc))]
pub use ios::*;

mod cocoa;
#[cfg(any(feature = "cocoa", doc))]
pub use cocoa::*;

mod win_ui_3;
#[cfg(any(feature = "winui3", doc))]
pub use win_ui_3::*;

mod gtk;
#[cfg(any(feature = "gtk", doc))]
pub use gtk::*;

mod qt;
#[cfg(any(feature = "qt", doc))]
pub use qt::*;
pub use tuple_list::{tuple_list as systems_list, TupleList as SystemsList};

/// Structs implementing a SystemDefinition are responsible to implement the
/// 'HasFoo' Traits, indicating that a specific System has the 'Foo' widget
/// available. ## Minimal example
///
/// TODO:
///
/// From here you are expected to implement the appropriate 'HasFoo' Traits.
///
/// For a list of those that ship with plating see [types
/// module](`crate::types`) or look at the [marker macro](`crate::marker`) to
/// create your own. ## Availability
/// SystemDefinitions need to be available cross-platform. They enable us to
/// check the availability of certain widgets on a single system.
///
/// Said that, [`SystemDefinition`]s shipped with plating are behind cfg flags
/// (like: 'gtk'). # See also
/// - [Tag<Sys>](`crate::tags::Tag`) for information to query widget
///   availability for all Systems in a
///   [systems_list](`crate::systems::systems_list`).
/// - [`crate::marker`] if you want to create your own 'HasFoo' Trait.
#[doc(notable_trait)]
pub trait SystemDefinition {}

#[allow(rustdoc::missing_doc_code_examples)]
impl<Head> SystemDefinition for (Head, ()) where Head: SystemDefinition {}

#[allow(rustdoc::missing_doc_code_examples)]
impl<Head, Tail> SystemDefinition for (Head, Tail)
where
    Head: SystemDefinition,
    Tail: SystemDefinition + SystemsList,
{
}

/// A trait that all `System`s need to implement.
///
/// It does not provide any functionality and is only a 'marker' trait
/// to mark the implementing struct as a System.
///
/// Example:
/// ```
/// use plating_systems::systems::{System, SystemDefinition};
///
/// struct BackendDefinition {}
/// // See [`SystemDefinition`] for more information.
/// impl SystemDefinition for BackendDefinition {}
///
///
/// struct Backend {}
///
/// impl System for Backend {
///     type Definition = BackendDefinition;
/// }
/// ```
/// A System is supposed to provide the Types the user can use.
/// In order to provide these, you have to implement the 'HasFoo' traits.
/// You can create your own with the [marker macro](`crate::marker`) or choose
/// one from the [types module](`crate::types`)
///
/// ## Availability
/// All native systems that come with plating are usually behind a cfg flag,
/// like e.g. 'gtk'.
///
/// Systems implement 'HasFoo' traits that require platform specific types for
/// the specific backend/platform and are therefore not available on all
/// platforms. That why the [`SystemDefinition`] trait exists. Use it to find
/// out if a type is available, use [`System`] to actually get that type.
///
/// There are exceptions to this rule, like the [Mock](`crate::systems::Mock`)
/// System, that is available on all platforms.
///
/// ## Misc
/// If you want to get information about a [`System`], use the
/// [`SystemDefinition`] available via [`System::Definition`].
///
/// See [`SystemDefinition`] for more information.
#[doc(notable_trait)]
pub trait System {
    /// The [`SystemDefinition`] that describes this System
    type Definition: SystemDefinition;
}
