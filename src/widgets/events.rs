/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::data::Vec2;
use crate::features::serde::{Deserialize, Serialize};
use crate::prelude::Named;

pub trait Event<'a> {
    fn timestamp(&self) -> u64; //TODO: better

    fn target(&self) -> &'a dyn Named; //todo: better
}

pub use crate::widgets::cocoa::event::CocoaKeyModifiers as KeyModifiers;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PressedEventType {
    Down,
    Pressed,
    Up,
}

pub trait KeyboardEventData {
    fn keyCode(&self) -> u16;
    fn characters(&self) -> Option<String>;
    fn modifiers(&self) -> KeyModifiers;
}

pub trait MouseEventData {
    fn location(&self) -> Vec2<u32>;
}

pub trait MouseMoveEventData {
    fn distance(&self) -> Vec2<u32>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DragEventType {
    DragStart,
    Dragging,
    DragStop,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HoverEventType {
    Enter,
    Hover,
    Leave,
}

/////events

pub trait MouseScrollEventData<'a>: MouseMoveEventData + MouseEventData + Event<'a> {}

pub trait MousePressEventData<'a>: MouseEventData + Event<'a> {
    fn state(&'a self) -> PressedEventType;
    fn button(&'a self) -> u8;
}

pub trait MouseDragEventData<'a>: MouseMoveEventData + MouseEventData + Event<'a> {
    fn drag_type(&self) -> DragEventType;
}

pub trait MouseHoverEventData<'a>: MouseMoveEventData + MouseEventData + Event<'a> {
    fn hover_type(&self) -> HoverEventType;
}
