/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use std::cmp::Ordering;
use std::ffi::CStr;

use cocoa::foundation::NSComparisonResult;
use objc::*;

use crate::backend::cocoa::base::{id, nil, BOOL, NO};
use crate::utils::{DeserializeEx, SerializeTrait};

// todo: implement in cocoa-rs
#[inline]
fn ordering_into(ord: NSComparisonResult) -> Ordering {
    match ord {
        NSComparisonResult::NSOrderedAscending => Ordering::Less,
        NSComparisonResult::NSOrderedSame => Ordering::Equal,
        NSComparisonResult::NSOrderedDescending => Ordering::Greater,
    }
}

#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct NSString(id);

impl NSString {
    #[inline]
    pub fn compare(&self, other: &Self) -> NSComparisonResult {
        unsafe { msg_send![self.0, compare:other.0] }
    }

    #[inline]
    pub fn compare_str(&self, other: &str) -> NSComparisonResult {
        use cocoa::foundation::NSString;

        unsafe {
            let other = NSString::alloc(nil).init_str(other);
            msg_send![self.0, compare: other]
        }
    }

    pub fn to_string(&self) -> String {
        use cocoa::foundation::NSString;

        unsafe { CStr::from_ptr(self.0.UTF8String()) }
            .to_string_lossy()
            .into_owned()
    }
}

impl From<NSString> for id {
    #[inline]
    fn from(obj: NSString) -> Self {
        obj.0
    }
}

impl PartialEq<str> for NSString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        use cocoa::foundation::NSString;
        unsafe { self.0.isEqualToString(other) }
    }
}

impl PartialEq for NSString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let rv: BOOL = unsafe { msg_send![self.0, isEqualToString:other.0] };
        rv != NO
    }
}

impl PartialOrd for NSString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(ordering_into(self.compare_str(other)))
    }
}

impl Ord for NSString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        ordering_into(self.compare(other))
    }
}

impl Eq for NSString {}

impl From<&str> for NSString {
    #[inline]
    fn from(s: &str) -> Self {
        use cocoa::foundation::NSString;

        unsafe { Self(NSString::alloc(nil).init_str(s)) }
    }
}

impl From<&String> for NSString {
    #[inline]
    fn from(s: &String) -> Self {
        use cocoa::foundation::NSString;

        unsafe { Self(NSString::alloc(nil).init_str(s)) }
    }
}

impl From<&NSString> for String {
    #[inline]
    fn from(s: &NSString) -> Self {
        s.to_string()
    }
}



impl std::fmt::Debug for NSString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

impl std::fmt::Display for NSString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        unsafe { NSString(msg_send![self.0, copy]) }
    }
}

impl SerializeTrait for NSString {
    #[cfg(feature = "serde")]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

use std::fmt;


#[cfg(feature = "serde")]
struct NSStringVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for NSStringVisitor {
    type Value = NSString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string.")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NSString::from(&value))
    }
}

impl<'de> DeserializeEx<'de> for NSString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(NSStringVisitor)
    }
}
