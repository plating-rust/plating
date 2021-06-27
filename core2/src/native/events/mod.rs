/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::utils::{Deserialize, Serialize};

pub trait Respondable
where
    Self: std::fmt::Debug + std::fmt::Display,
{
}

//Who gets it: bubble up
//Returns: EventResult
pub trait UserEvent: Respondable {
    //target
}

// Can be (each its own struct):
//get generic attributes
//implements native trait to get platform specific attributes
/*
MouseDown(),
MouseUp(),
MouseDragged(),
MouseMove(),
MouseEnter(),
MouseLeave(),

MouseClick(),
MouseDoubleClick(),

MouseWheel(),

KeyboardPress(),
KeyboardDown(),
KeyboardUp(),

KeyboardModifierChange(),*/


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Next<T: Respondable> {
    //T can be an UserEvent, Action or Intention
    Next(T),
    Final,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EventResult<T: Respondable> {
    Unhandled(),
    Handled(Next<T>), //T can be a UserEvent, Action or Intention
    Error(),
}

//all elements
pub enum SystemEvent {
    Tick,
    DisplayChange,
    LanguageChange,
    ThemeChange,
    SettingChange,
}

//for each individual element
pub enum Lifecycle {
    Setup,
    Connect,
    Disconnect,
    Teardown,
}

//no bubbling

//examples
//focus
//unfocus
//resize (on windows)
//fullscreen (on windows)
//minimize (on windows)
//close
//execute (on button, menu item, etc)
//data change (form fields)
pub trait Action {}

//bubbling up
pub trait Intention {}
