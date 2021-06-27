/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

mod system;
pub use system::Backend;

mod app;
pub use app::{AppOutlet, AppWidget};

mod window;
pub use window::{WindowOutlet, WindowWidget};

mod menu;
pub use menu::{MenuOutlet, MenuWidget};

mod widget;
pub use widget::{WidgetAbstractionLevel, WidgetBuilder};


pub mod properties;
//pub use parameters::Parameters;

mod template;
pub use template::Template;

#[cfg(target_os = "macos")]
pub mod cocoa {
    pub use super::system::cocoa::Cocoa;
}

pub mod prelude {
    pub use super::app::AppWidget;
    pub use super::menu::MenuWidget;
    pub use super::system::prelude::*;
    pub use super::window::WindowWidget;
    pub use super::WidgetBuilder;
}
