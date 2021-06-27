/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(async_stream)]
#![feature(generic_associated_types)]

mod children;
mod events;
mod settings;

/*
struct KeyEvent {}
pub struct EditField<ClickEventResult, StreamType>
where
    StreamType: std::stream::Stream<Item = KeyEvent>,
{
    _d: std::marker::PhantomData<ClickEventResult>,
    _e: std::marker::PhantomData<StreamType>,
}
impl<ClickEventResult, StreamType> EditField<ClickEventResult, StreamType>
where
    StreamType: std::stream::Stream<Item = KeyEvent>,
{
    fn new() -> Self {
        Self {
            _d: Default::default(),
            _e: Default::default(),
        }
    }
}
impl<ClickEventResult, StreamType> ClickHandler<ClickEventResult, StreamType> for EditField<ClickEventResult, StreamType>
where
    StreamType: std::stream::Stream<Item = KeyEvent>,
{
    fn key_pressed_handler<F>(&self, mut func: F)
        where
            F: FnMut(&StreamType) -> std::stream::Stream<Item = ClickEventResult>, {
        //...
        todo!{}
    }
}

enum Actions {
    //...
    UpdateAutocomplete,
    ValidateForm,
    //...
}

trait ClickHandler<ClickEventResult, StreamType>
where
    StreamType: std::stream::Stream<Item = KeyEvent>,
{
    fn key_pressed_handler<F>(&self, func: F)
        where
            F: FnMut(&StreamType) -> std::stream::Stream<Item = ClickEventResult>;
}

trait SizedStream: std::stream::Stream + Sized {}
*/

fn main() -> plating::PlatingResult<()> {
    settings::main()?;

    events::main()?;

    children::main()?;

    Ok(())
}
