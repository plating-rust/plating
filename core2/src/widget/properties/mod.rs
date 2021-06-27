/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */
use super::{Backend, WindowWidget};
use crate::native::NativeWidget;
use crate::utils::{Deserialize, Property, Serialize};
use crate::widget::WidgetAbstractionLevel;
use crate::PlatingResult;

pub trait AssociatedType {
    type ASSOC;
}
impl<T> AssociatedType for T {
    type ASSOC = T;
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Label(pub String);

impl WidgetAbstractionLevel for Label {}

impl<STATE, T, BACKEND> Property<STATE, T, BACKEND> for Label
where
    T: WindowWidget<STATE, BACKEND> + NativeWidget<STATE, Backend = BACKEND>,
    BACKEND: Backend,
{
    fn provide(&self, target: &mut T) -> PlatingResult<()> {
        target.set_label(self)
    }
}
