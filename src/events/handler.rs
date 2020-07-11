/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};

/// #User Events like:
///     -   MouseClick,
///     -   KeyPress,
///     -   ScrollWheel,
///     -   DblClick,
///     -   Hover
///
/// call ALL before listeners
/// call one handler after the other until someone handled it
///     completely abort on error
/// call ALL after listeners
///
/// if not handled, bubble up

/// # Lifecycle Actions like: inject, detach
///
/// call ALL before listeners
/// call the one handler returning OK or Error for inject/detach
/// call ALL after listeners

/// # System Actions like:
/// - Resize
/// - Fullscreen
/// - Focus
///
/// call All before listeners
/// call the one handler returning the new value for the system
/// call All after listeners
///
/// bubble down the after listeners

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum PermissionState {
    ALLOWED,
    DENIED,
}
impl Default for PermissionState {
    fn default() -> Self {
        Self::ALLOWED
    }
}

/// Enum representing the EventState after a Event Callback was called.
///
/// # Example
/// todo: example callback return handled and unhandled on some condition
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum HandledState<T = ()> {
    /// this event has not yet been handled.
    UNHANDLED,
    /// represent that the event was handled and no further event handlers should be called
    HANDLED(T),
    // map this action to a message that is dispatched
    //todo: MAP(Message)
}
impl<T> Default for HandledState<T> {
    fn default() -> Self {
        Self::UNHANDLED
    }
}

pub type PermissionResult = Result<PermissionState, anyhow::Error>;
pub type HandlerResult<T> = Result<HandledState<T>, anyhow::Error>;
