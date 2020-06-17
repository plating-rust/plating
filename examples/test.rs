/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use plating::prelude::*;
#[cfg(target_os = "macos")]
use plating::widgets::cocoa::{CocoaButton, CocoaButtonParameters};
use plating::widgets::generic::{Root, RootParameters, Window, WindowParameters};
#[cfg(target_os = "win")]
use plating::widgets::win::{WinButton, WinButtonParameters};

fn main() {
    // create a *generic* root element
    let mut root = Root::new(RootParameters::default()).unwrap();
    // create a *generic* window element
    let mut window = Window::new(WindowParameters::default()).unwrap();

    // create a *native* element for more control
    #[cfg(target_os = "macos")]
    let button: CocoaButton = CocoaButton::new(CocoaButtonParameters::default()).unwrap();
    #[cfg(target_os = "win")]
    let button: WinButton = WinButton::new(WinButtonParameters::default()).unwrap();

    //mix and match them together
    window.add_child(button).unwrap();
    root.add_child(window).unwrap();
}
