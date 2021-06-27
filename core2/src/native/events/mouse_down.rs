/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub trait EmitsMouseDown {
    pub fn setup(&mut self) -> Stream<u8>;
}

pub trait MouseDownAvailable {
    pub fn setup(&mut self) -> Stream<u8>;
}

impl<T> MouseDownAvailable for T
where
    T: EmitsMouseDown
{
    fn setup(&mut self) -> Stream<u8> {
        <Self as EmitsMouseDown>::setup(self)
    }
}