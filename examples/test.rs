/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use plating::prelude::*;
use plating::widgets::root::RootParameters;
use plating::widgets::window::{WindowChildren, WindowParameters};
use plating::widgets::{default_system, System};

#[cfg(target_os = "macos")]
use plating::widgets::cocoa::{CocoaButton, CocoaButtonParameters};
#[cfg(target_os = "win")]
use plating::widgets::win::{WinButton, WinButtonParameters};

fn main() {
    // create a *generic* root element
    let mut root = <default_system as System>::RootType::new(RootParameters::default()).unwrap();
    // create a *generic* window element
    let mut window =
        <default_system as System>::WindowType::new(WindowParameters::default()).unwrap();

    // create a *native* element for more control
    #[cfg(target_os = "macos")]
    let button: CocoaButton = CocoaButton::new(CocoaButtonParameters::default()).unwrap();
    #[cfg(target_os = "win")]
    let button: WinButton = WinButton::new(WinButtonParameters::default()).unwrap();

    //mix and match them together
    Outlet::<WindowChildren>::add_child(&mut window, button).unwrap();
    root.add_child(window).unwrap();
}
