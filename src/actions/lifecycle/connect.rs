/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::actions::Identity;
use crate::events::ListenerType;
use crate::features::serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConnectEvent {
    connected: bool,
}

pub trait ConnectTopic {
    fn add_listener(
        &self,
        when: ListenerType,
        handler: Box<impl FnMut(&ConnectEvent, &dyn Identity)>,
    );
}
