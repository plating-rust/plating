/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

//! Module containing basic data types used throughout plating 

use crate::features::serde::{Deserialize, Serialize};

/// Basic 2d Vector. 
/// Implemented as a Tuple. No specific additional features.
pub type Vec2<T> = (T, T);
/// Basic 2d Vector. 
/// Implemented as a Tuple. No specific additional features.
pub type Vec3<T> = (T, T, T);

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CheckedState {
    Off,
    On,
    Mixed,
}
impl Default for CheckedState {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

///Specifies a rectangular area.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
pub struct Rect {
    ///the position of the top left of this rectangular area.
    /// 
    /// notation: (x, y)
    top_left: Vec2<i32>,
    /// The size of this rectangular area.
    /// 
    /// notation: (width, height)
    size: Vec2<i32>,
}

/// Data representing a rgba color.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
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
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {r, g, b, a}
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
    pub fn black() -> RGBA {
        RGBA::new(0, 0, 0, 0)
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
    pub fn white() -> RGBA {
        RGBA::new(255, 255, 255, 0)
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
    pub fn transparent() -> RGBA {
        RGBA::new(0, 0, 0, 255)
    }
}
/// Converts a RGB Color to an RGBA color by keeping the rgb part and
/// setting the alpha value to `0`.
impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> RGBA {
        RGBA::new(rgb.r, rgb.g, rgb.b, 0)
    }
}

/// Data representing a RGB color.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
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
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB {r, g, b}
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
    pub fn black() -> RGB {
        RGB::new(0, 0, 0)
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
    pub fn white() -> RGB {
        RGB::new(255, 255, 255)
    }
}

/// Abstract enum representing a Color.
/// 
/// Can be represented either via [`RGB`] or [`RGBA`].
//todo: allow changing between different types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
pub enum Color {
    /// Stores an RGBA value
    RGBA(RGBA),
    /// Stores an RGB value
    RGB(RGB)
}
/// Implements the default trait.
/// 
/// Defaults to a *black* [`RGB`] color.
impl Default for Color {
    fn default() -> Color {
        Self::RGB(RGB::black())
    } 
}
