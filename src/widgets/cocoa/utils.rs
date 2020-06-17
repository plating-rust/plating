/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};

pub fn make_ns_string(s: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}

#[cfg(feature = "serde")]
pub(super) mod serde {
    use cocoa::appkit::NSWindowStyleMask;

    use serde::{de::Visitor, Deserializer, Serializer};

    //todo: macroize!
    pub fn serialize_ns_window_style_mask<S>(
        optional_mask: &Option<NSWindowStyleMask>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match optional_mask {
            Some(mask) => serializer.serialize_u64(mask.bits()),
            None => serializer.serialize_none(),
        }
    }

    //todo: macroize!
    pub fn deserialize_ns_window_style_mask<'de, D>(
        d: D,
    ) -> Result<Option<NSWindowStyleMask>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Option<NSWindowStyleMask>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("None or ")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(
                    NSWindowStyleMask::from_bits(value), //todo: ignores error and treats it at none
                )
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        d.deserialize_identifier(FieldVisitor)
    }
}
