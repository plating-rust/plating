/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub use tuple_list::{tuple_list as setting_list, TupleList as SettingsList};

use crate::native::NativeWidget;
use crate::utils::{DeserializeTrait, SerializeTrait};
use crate::widget::Backend;
use crate::PlatingResult;

//todo: implement this for settings_list
//also implement #[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
//for settings list

pub trait Property<STATE, TARGET, BACKEND>
where
    Self: Sized + PartialEq + DeserializeTrait + SerializeTrait,
    BACKEND: Backend,
{
    fn provide(&self, target: &mut TARGET) -> PlatingResult<()>;
}


//todo: some generic system!{} macro
impl<STATE, TARGET, BACKEND> Property<STATE, TARGET, BACKEND> for ()
where
    TARGET: NativeWidget<STATE, Backend = BACKEND>,
    BACKEND: Backend,
{
    fn provide(&self, _target: &mut TARGET) -> PlatingResult<()> {
        //nothing todo
        Ok(())
    }
}

impl<STATE, TARGET, BACKEND, Head, Tail> Property<STATE, TARGET, BACKEND> for (Head, Tail)
where
    BACKEND: Backend,
    Head: Property<STATE, TARGET, BACKEND>,
    TARGET: NativeWidget<STATE, Backend = BACKEND>,
    Tail: Property<STATE, TARGET, BACKEND> + SettingsList,
{
    fn provide(&self, target: &mut TARGET) -> PlatingResult<()> {
        self.0.provide(target)?;
        self.1.provide(target)?;

        Ok(())
    }
}
