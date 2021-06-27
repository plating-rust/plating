/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::backend::cocoa::foundation::NSString;
use crate::native::cocoa::{CocoaApp, CocoaAppWidget, CocoaWindow, CocoaWindowWidget};
use crate::native::{Native, ToNative};
use crate::utils::{ChildrenList, Deserialize, Property, Serialize};
use crate::widget::cocoa::Cocoa;
use crate::widget::properties::Label;
use crate::PlatingResult;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct CocoaLabel(NSString);



impl Native<Cocoa> for CocoaLabel {}

impl ToNative<Cocoa> for Label {
    type Result = CocoaLabel;

    fn to_native(&self) -> Self::Result {
        CocoaLabel((&self.0).into())
    }
}

impl std::fmt::Display for CocoaLabel {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<STATE, CHILDREN, MENU> Property<STATE, CocoaWindowWidget<STATE, CHILDREN, MENU>, Cocoa>
    for CocoaLabel
where
    CHILDREN: ChildrenList,
    MENU: ChildrenList,
{
    fn provide(&self, target: &mut CocoaWindowWidget<STATE, CHILDREN, MENU>) -> PlatingResult<()> {
        target.set_label(self)
    }
}

impl<STATE, CHILDREN> Property<STATE, CocoaAppWidget<STATE, CHILDREN>, Cocoa> for CocoaLabel
where
    CHILDREN: ChildrenList,
{
    fn provide(&self, target: &mut CocoaAppWidget<STATE, CHILDREN>) -> PlatingResult<()> {
        target.set_label(self)
    }
}
