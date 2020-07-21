/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::features::serde::{Deserialize, Serialize};
use crate::widgets::button::ButtonParameters;
use crate::widgets::utils::Parameters;

pub trait CocoaButtonPlatformParameters {}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct CocoaButtonParameters {
    label: Option<String>,
}

impl Parameters for CocoaButtonParameters {
    fn merge(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        if self.label().is_none() {
            self.set_label_optionally(rhs.label);
        }

        Ok(())
    }
    fn on_top(&mut self, rhs: Self) -> Result<(), anyhow::Error> {
        self.set_label_optionally(rhs.label);

        Ok(())
    }
}

impl ButtonParameters for CocoaButtonParameters {
    fn label(&self) -> &Option<String> {
        &self.label
    }
    fn set_label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }
    fn set_label_optionally(&mut self, label: Option<String>) -> &mut Self {
        if let Some(s) = label {
            self.set_label(s);
        }
        self
    }
    fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }
}

impl CocoaButtonPlatformParameters for CocoaButtonParameters {}
