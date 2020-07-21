/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

#![deny(missing_docs)]

//! Module containing basic data types used throughout plating

use crate::features::serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Basic 2d Vector.
/// Implemented as a Tuple. No specific functions.
pub type Vec2<T> = (T, T);
/// Basic 2d Vector.
/// Implemented as a Tuple. No specific functions.
pub type Vec3<T> = (T, T, T);

/// The state of checkable objects like e.g. check boxes.
///
/// See also [`OptionalCheckedState`]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CheckedState {
    /// This items state is 'not checked'
    Off,
    /// This items state is 'checked'
    On,
    /// For items that have several sub items,
    /// this state means some children are checked, some are not.
    Mixed,
}
/// Defaults to the [`Off`](CheckedState::Off) state
impl Default for CheckedState {
    fn default() -> Self {
        Self::Off
    }
}
/// Tries to convert from OptionalCheckedState.
///
/// Succeeds for [`OptionalCheckedState::Off`], [`OptionalCheckedState::On`] and [`OptionalCheckedState::Mixed`]
///
/// Fails for [`OptionalCheckedState::None`]
impl TryFrom<OptionalCheckedState> for CheckedState {
    type Error = &'static str;

    fn try_from(state: OptionalCheckedState) -> Result<Self, Self::Error> {
        match state {
            OptionalCheckedState::None => Err("Invalid OptionalCheckedState: None"),
            OptionalCheckedState::Off => Ok(Self::Off),
            OptionalCheckedState::On => Ok(Self::On),
            OptionalCheckedState::Mixed => Ok(Self::Mixed),
        }
    }
}

/// The state of optionally checkable objects like menu items.
/// The difference to [`CheckedState`] is that this supports are 'None' option.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum OptionalCheckedState {
    /// This item has its checked state turned off
    None,
    /// Same as [`CheckedState::Off`]
    Off,
    /// Same as [`CheckedState::On`]
    On,
    /// Same as [`CheckedState::Mixed`]
    Mixed,
}
/// Defaults to [`None`](OptionalCheckedState::None) State
impl Default for OptionalCheckedState {
    fn default() -> Self {
        Self::None
    }
}
impl From<CheckedState> for OptionalCheckedState {
    fn from(state: CheckedState) -> Self {
        match state {
            CheckedState::Off => Self::Off,
            CheckedState::On => Self::On,
            CheckedState::Mixed => Self::Mixed,
        }
    }
}

/// Generic Enum for representing a horizontal direction.
///
/// Can be used to represent text direction as well as layout direction
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Direction {
    ///
    LeftToRight,
    ///
    RightToLeft,
}

/// Specifies a rectangular area.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Rect {
    /// the position of the top left of this rectangular area.
    ///
    /// notation: (x, y)
    top_left: Vec2<i32>,
    /// The size of this rectangular area.
    ///
    /// notation: (width, height)
    size: Vec2<i32>,
}

/// Data representing a rgba color.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct RGBA {
    /// stores the red part of the color.
    ///
    /// Values: `0` - `255`
    pub r: u8,
    /// stores the green part of the color.
    ///
    /// values: `0` - `255`
    pub g: u8,
    /// stores the blue part of the color.
    ///
    /// values: `0` - `255`
    pub b: u8,
    /// stores the alpha part of the color.
    ///
    /// values: `0` - `255`<br>
    /// `0` means not transparent, `255` mean fully transparent.
    pub a: u8,
}
impl RGBA {
    /// constructor taking all 4 components of a RGBA Color
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a black RGBA color.
    /// non-transparent
    ///
    /// ```rust
    /// use plating::RGBA;
    /// let value = RGBA::black();
    ///
    /// assert_eq!(value, RGBA::new(0, 0, 0, 0));
    /// ```
    #[inline]
    pub const fn black() -> Self {
        Self::new(0, 0, 0, 0)
    }
    /// Creates a white RGBA color.
    /// non-transparent
    ///
    /// ```rust
    /// use plating::RGBA;
    /// let value = RGBA::white();
    ///
    /// assert_eq!(value, RGBA::new(255, 255, 255, 0));
    /// ```
    #[inline]
    pub const fn white() -> Self {
        Self::new(255, 255, 255, 0)
    }

    /// Create a transparent color.
    ///
    /// RGB values are `0` (black)<br>
    /// Alpha is `1`
    ///
    /// ```rust
    /// use plating::RGBA;
    ///
    /// let value = RGBA::transparent();
    ///
    /// assert_eq!(value, RGBA::new(0, 0, 0, 255));
    /// ```
    #[inline]
    pub const fn transparent() -> Self {
        Self::new(0, 0, 0, 255)
    }
}
/// Converts a RGB Color to an RGBA color by keeping the rgb part and
/// setting the alpha value to `0`.
impl From<RGB> for RGBA {
    #[inline]
    fn from(rgb: RGB) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, 0)
    }
}

/// Data representing a RGB color.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct RGB {
    /// stores the red part of the color.
    ///
    /// Values: `0` - `255`
    r: u8,
    /// stores the green part of the color.
    ///
    /// values: `0` - `255`
    g: u8,
    /// stores the blue part of the color.
    ///
    /// values: `0` - `255`
    b: u8,
}
impl RGB {
    /// constructor taking all 3 components of a RGBA Color
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a black RGB color.
    ///
    /// ```rust
    /// use plating::RGB;
    ///
    /// let value = RGB::black();
    ///
    /// assert_eq!(value, RGB::new(0, 0, 0));
    /// ```
    #[inline]
    pub const fn black() -> Self {
        Self::new(0, 0, 0)
    }
    /// Creates a white RGB color.
    ///
    /// ```rust
    /// use plating::RGB;
    ///
    /// let value = RGB::white();
    ///
    /// assert_eq!(value, RGB::new(255, 255, 255));
    /// ```
    #[inline]
    pub const fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

/// Abstract enum representing a Color.
///
/// Chose between an [`RGB`] or [`RGBA`] presentation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum Color {
    /// Stores an RGBA value
    RGBA(RGBA),
    /// Stores an RGB value
    RGB(RGB),
}
/// Implements the default trait.
///
/// Defaults to a *black* [`RGB`] color.
impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::RGB(RGB::black())
    }
}
