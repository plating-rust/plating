/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::generic::WindowHandlerTrait;

#[derive(Debug)]
pub struct CocoaWindowDelegate {}

impl CocoaWindowDelegate {
    pub fn new() -> CocoaWindowDelegate {
        CocoaWindowDelegate {}
    }
}

impl WindowHandlerTrait for CocoaWindowDelegate {
    fn setResizeHandler(&mut self, handler: Box<impl FnMut()>) {
        todo!()
    }
    fn addResizeListener(
        &mut self,
        when: crate::widgets::generic::ListenerType,
        handler: Box<impl FnMut()>,
    ) {
        todo!()
    }
}
