/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use crate::native::NativeWidget;
use crate::prelude::Backend;
use crate::utils::{ChildrenList, Deserialize, Serialize};
use crate::PlatingResult;

#[derive(Debug, Clone, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Template<WIDGET, STATE, CHILDREN, BACKEND>
where
    WIDGET: NativeWidget<STATE, Backend = BACKEND>,
    STATE: Default + Clone,
    CHILDREN: ChildrenList,
    BACKEND: Backend,
    //SETTINGS: SettingsList + WidgetLevel + ToNative<Backend = BACKEND>
{
    parent: Option<Box<Template<WIDGET, STATE, CHILDREN, BACKEND>>>,
    state:  Option<STATE>,

    //settings: Option<SETTINGS>,
    //children: Vec<Template<WIDGET>>,
    _widget:  std::marker::PhantomData<WIDGET>,
    _backend: std::marker::PhantomData<BACKEND>,

    _children: std::marker::PhantomData<CHILDREN>,
}
impl<WIDGET, STATE, CHILDREN, BACKEND> Template<WIDGET, STATE, CHILDREN, BACKEND>
where
    WIDGET: NativeWidget<STATE, Backend = BACKEND>,
    STATE: Default + Clone,
    CHILDREN: ChildrenList,
    BACKEND: Backend,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> PlatingResult<WIDGET> {
        let _state = match self.state.clone() {
            Some(state) => state,
            None => STATE::default(),
        };
        /*
        let settings = match self.calculated_settings() {
            Some(s) => s,
            None => Default::default(),
        };*/

        //WIDGET::new_with_state(state, &())
        todo! {}
    }

    pub fn parent(&self) -> &Option<Box<Template<WIDGET, STATE, CHILDREN, BACKEND>>> {
        &self.parent
    }

    pub fn set_parent_option(
        &mut self,
        parent: Option<Box<Template<WIDGET, STATE, CHILDREN, BACKEND>>>,
    ) -> &mut Self {
        self.parent = parent;
        self
    }

    pub fn unset_parent(&mut self) -> &mut Self {
        self.parent = None;
        self
    }

    pub fn set_parent<PARENT: Into<Box<Self>>>(&mut self, parent: PARENT) -> &mut Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn state(&self) -> Option<STATE>
    where
        STATE: Clone,
    {
        self.state.clone().or(match &self.parent {
            Some(parent) => parent.state(),
            None => None,
        })
    }

    pub fn set_state_option(&mut self, state: Option<STATE>) -> &mut Self {
        self.state = state;
        self
    }

    pub fn unset_state(&mut self) -> &mut Self {
        self.state = None;
        self
    }

    pub fn set_state<IState: Into<STATE>>(&mut self, state: IState) -> &mut Self {
        self.state = Some(state.into());
        self
    }
    /*
    pub fn calculated_settings(&self) -> Option<<WIDGET as Widget<STATE>>::Params> {
        match (&self.settings, &self.parent) {
            (Some(s), None) => Some(s.to_owned()),
            (Some(s1), Some(p)) => match &p.settings {
                Some(s2) => Some(<WIDGET as Widget<STATE>>::Params::merge(&s2, &s1)),
                None => Some(s1.to_owned()),
            },
            (None, Some(p)) => match &p.settings {
                Some(s) => Some(s.to_owned()),
                None => None,
            },
            (None, None) => None,
        }
    }

    pub fn settings(&self) -> &Option<<WIDGET as Widget<STATE>>::Params> {
        &self.settings
    }

    pub fn mut_settings(&mut self) -> &mut Option<<WIDGET as Widget<STATE>>::Params> {
        &mut self.settings
    }

    pub fn unset_settings(&mut self) -> &mut Self {
        self.settings = None;
        self
    }

    pub fn set_settings_option<P: Into<<WIDGET as Widget<STATE>>::Params>>(
        &mut self,
        settings: Option<P>,
    ) -> &mut Self {
        self.settings = settings.map(|o| o.into());
        self
    }

    pub fn set_settings<P: Into<<WIDGET as Widget<STATE>>::Params>>(
        &mut self,
        settings: P,
    ) -> &mut Self {
        self.settings = Some(settings.into());
        self
    }*/
}

impl<WIDGET, STATE, CHILDREN, BACKEND> Default for Template<WIDGET, STATE, CHILDREN, BACKEND>
where
    WIDGET: NativeWidget<STATE, Backend = BACKEND>,
    STATE: Default + Clone,
    CHILDREN: ChildrenList,
    BACKEND: Backend,
{
    fn default() -> Self {
        Self {
            parent:    None,
            state:     None,
            _widget:   std::marker::PhantomData,
            _backend:  std::marker::PhantomData,
            _children: std::marker::PhantomData,
            //settings: None,
        }
    }
}
