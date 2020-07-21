/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::widgets::events::{
    MouseDragEventData, MouseHoverEventData, MousePressEventData, MouseScrollEventData,
};

pub trait EventSystem<'a> {
    ////////////////////////
    // Input events
    ////////////////////////
    //cc: hover_handler(ev: EventData, target: Widget) -> HANDLED
    type MouseHoverEvent: MouseHoverEventData<'a>;
    //cc: drag_handler(ev: EventData, current_target: Widget, start_target: Widget) -> HANDLED
    type MouseDragEvent: MouseDragEventData<'a>;
    //cc: press_handler(ev: EventData, target: Widget) -> HANDLED
    type MousePressEvent: MousePressEventData<'a>;
    //cc: scroll_handler(ev: EventData, target: Widget) -> HANDLED
    type MouseScrollEvent: MouseScrollEventData<'a>;

    //type KeyPressedEvent: ;

    ////////////////////////
    // Lifecycle events (not from backend)
    ////////////////////////
    //todo: really needed? create(Settings) -> Result<Settings, Error>
    //todo: inject(Parent) -> Result<(), Error>
    //todo: detach() -> Result<(), Error>
    //todo: really needed? destroy() -> ()

    //todo: child_inject(child) -> Result<(), Error>
    //todo: child_detach(child) -> Result<(), Error>
}
