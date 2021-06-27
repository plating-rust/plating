/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::utils::{Deserialize, Serialize};


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stateless {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Border {
    BORDER     = 1,
    BORDERLESS = 0,
}

//Color

//Font

//ID

//Size
//Percentage / Pixel
//Width / height

//Label
