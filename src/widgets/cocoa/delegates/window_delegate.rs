/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::events::ListenerType;
use crate::widgets::window::WindowHandlerTrait;

#[derive(Debug, Default, Clone)]
pub(crate) struct CocoaWindowDelegate {}

impl CocoaWindowDelegate {
    pub fn new() -> CocoaWindowDelegate {
        CocoaWindowDelegate {}
    }
}

impl WindowHandlerTrait for CocoaWindowDelegate {
    fn set_resize_handler(&mut self, _handler: Box<impl FnMut()>) {
        todo!()
    }
    fn add_resize_listener(&mut self, _when: ListenerType, _handler: Box<impl FnMut()>) {
        todo!()
    }
}
