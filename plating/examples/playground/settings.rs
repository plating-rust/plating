/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub mod module {

    //#[cfg(feature = "serde")]
    //pub(crate) use serde::{Deserialize, Serialize};
    pub use plating::utils::settings::{setting_list, SettingsList};
    use plating::PlatingResult;

    //////////utils
    pub trait ToNative {
        type CocoaResult;
        fn to_native() -> Self::CocoaResult;
    }

    ////////// utils/property
    pub trait Property: PartialEq {}

    impl Property for () {}

    impl<Head, Tail> Property for (Head, Tail)
    where
        Head: Property,
        Tail: Property + SettingsList,
    {
    }

    pub trait Provides<T: Property> {
        fn provide_opt(&self) -> Option<&T>;
    }
    impl<A: Property, B: Property> Provides<A> for B {
        default fn provide_opt(&self) -> Option<&A> {
            None
        }
    }

    //TODO: property macro



    ////////// native/property/A
    #[derive(Debug, Hash, Clone, Copy, Eq, PartialEq/*, Serialize, Deserialize*/)]
    pub struct NativeA(pub u32);
    impl Property for NativeA {}
    impl Provides<NativeA> for NativeA {
        fn provide_opt(&self) -> Option<&NativeA> {
            Some(self)
        }
    }
    pub trait AReceiver {
        fn set_a(&mut self, a: &NativeA) -> PlatingResult<()>;
    }

    ////////// {native/widget}property/B
    #[derive(Debug, Hash, Clone, Copy, Eq, PartialEq/*, Serialize, Deserialize*/)]
    pub struct NativeB(pub bool);
    impl Property for NativeB {}
    impl Provides<NativeB> for NativeB {
        fn provide_opt(&self) -> Option<&NativeB> {
            Some(self)
        }
    }
    pub trait BReceiver {
        fn set_b(&mut self, a: &NativeB) -> PlatingResult<()>;
    }

    ////////// {native/widget}property/C
    #[derive(Debug, Hash, Clone, Copy, Eq, PartialEq/*, Serialize, Deserialize*/)]
    pub struct NativeC(pub u32);
    impl Property for NativeC {}
    impl Provides<NativeC> for NativeC {
        fn provide_opt(&self) -> Option<&NativeC> {
            Some(self)
        }
    }
    pub trait CReceiver {
        fn set_c(&mut self, a: &NativeC) -> PlatingResult<()>;
    }

    ////////// {native/widget}property/S
    #[derive(Debug, Hash, Clone, Eq, PartialEq/*, Serialize, Deserialize*/)]
    pub struct NativeS(pub String);
    impl Property for NativeS {}
    impl Provides<NativeS> for NativeS {
        fn provide_opt(&self) -> Option<&NativeS> {
            Some(self)
        }
    }

    pub trait SReceiver {
        fn set_s(&mut self, s: &NativeS) -> PlatingResult<()>;
    }

    ////////// {native/widget}property/U
    #[derive(Debug, Hash, Clone, Copy, Eq, PartialEq/*, Serialize, Deserialize*/)]
    pub struct NativeU(pub u32);
    impl Property for NativeU {}
    impl Provides<NativeU> for NativeU {
        fn provide_opt(&self) -> Option<&NativeU> {
            Some(self)
        }
    }
    pub trait UReceiver {
        fn set_u(&mut self, u: &NativeU) -> PlatingResult<()>;
    }


    pub mod prelude {
        pub use super::{AReceiver, BReceiver, CReceiver, SReceiver, UReceiver};
    }


    impl CocoaWidgetPropertyProvider for () {
        fn provide(&self, _target: &mut CocoaWidget) -> PlatingResult<()> {
            Ok(())
        }
    }

    impl<Head, Tail> CocoaWidgetPropertyProvider for (Head, Tail)
    where
        Head: CocoaWidgetPropertyProvider,
        Tail: CocoaWidgetPropertyProvider + SettingsList,
    {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()> {
            self.0.provide(target)?;
            self.1.provide(target)
        }
    }



    impl<Head, Tail, T: Property> Provides<T> for (Head, Tail)
    where
        Head: Property,
        Tail: Property + SettingsList,
    {
        fn provide_opt(&self) -> Option<&T> {
            let r = self.0.provide_opt();
            if r.is_some() {
                return r;
            }
            self.1.provide_opt()
        }
    }


    // widgets
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CrossWidget {
        native: CocoaWidget,
    }

    impl CrossWidget {
        #[inline(always)]
        pub fn apply<SL: SettingsList + CocoaWidgetPropertyProvider>(
            &mut self,
            settings: SL,
        ) -> PlatingResult<()> {
            self.native.apply(settings)
        }

        #[inline(always)]
        pub fn apply_possible<SL: SettingsList + Property>(
            &mut self,
            settings: SL,
        ) -> PlatingResult<()> {
            self.native.apply_possible(settings)
        }

        #[inline(always)]
        pub fn new<SL: SettingsList + CocoaWidgetPropertyProvider>(
            settings: SL,
        ) -> PlatingResult<Self> {
            let r = Self {
                native: CocoaWidget::new(settings)?,
            };
            Ok(r)
        }
    }

    // native/cocoa/
    #[derive(Debug)]
    pub struct CocoaWidget {
        a: NativeA,
        b: NativeB,
        c: NativeC,
        s: NativeS,
    }
    impl CocoaWidget {
        pub fn apply<SL: SettingsList + CocoaWidgetPropertyProvider>(
            &mut self,
            settings: SL,
        ) -> PlatingResult<()> {
            settings.provide(self)
        }

        pub fn apply_possible<SL: SettingsList + Property>(
            &mut self,
            settings: SL,
        ) -> PlatingResult<()> {
            if let Some(a) = settings.provide_opt() {
                self.set_a(a)?
            }

            if let Some(b) = settings.provide_opt() {
                self.set_b(b)?
            }

            if let Some(c) = settings.provide_opt() {
                self.set_c(c)?
            }

            if let Some(s) = settings.provide_opt() {
                self.set_s(s)?
            }

            Ok(())
        }

        pub fn new<SL: SettingsList + CocoaWidgetPropertyProvider>(
            settings: SL,
        ) -> PlatingResult<Self> {
            let mut r = Self {
                a: NativeA(0),
                b: NativeB(false),
                c: NativeC(10),
                s: NativeS(String::from("Yay")),
            };
            r.apply(settings)?;
            Ok(r)
        }
    }

    pub trait CocoaWidgetPropertyProvider {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()>;
    }

    impl CocoaWidgetPropertyProvider for NativeA {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()> {
            target.set_a(self)
        }
    }
    impl CocoaWidgetPropertyProvider for NativeC {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()> {
            target.set_c(self)
        }
    }
    impl CocoaWidgetPropertyProvider for NativeS {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()> {
            target.set_s(self)
        }
    }
    impl CocoaWidgetPropertyProvider for NativeB {
        fn provide(&self, target: &mut CocoaWidget) -> PlatingResult<()> {
            target.set_b(self)
        }
    }

    impl AReceiver for CocoaWidget {
        fn set_a(&mut self, a: &NativeA) -> PlatingResult<()> {
            println!("a: {:?}", a);
            self.a = *a;
            Ok(())
        }
    }
    impl BReceiver for CocoaWidget {
        fn set_b(&mut self, b: &NativeB) -> PlatingResult<()> {
            self.b = *b;
            println!("b: {:?}", b);

            Ok(())
        }
    }
    impl CReceiver for CocoaWidget {
        fn set_c(&mut self, c: &NativeC) -> PlatingResult<()> {
            self.c = *c;
            println!("b: {:?}", c);
            Ok(())
        }
    }
    impl SReceiver for CocoaWidget {
        fn set_s(&mut self, s: &NativeS) -> PlatingResult<()> {
            self.s = s.clone();
            println!("s: {:?}", s);
            Ok(())
        }
    }
}


pub fn completely_native() -> plating::PlatingResult<()> {
    use module::{NativeA, NativeB, NativeS, NativeU};
    //use module::prelude::*;


    let mut a = module::CrossWidget::new(module::setting_list!())?;

    println!("{:?}", a);

    let settings = module::setting_list!(NativeA(2), NativeB(true), NativeS(String::from("abc")));

    let settings2 = module::setting_list!(
        NativeU(15),
        NativeA(105),
        NativeB(true),
        NativeS(String::from("wtf"))
    );


    a.apply(settings)?;
    println!("{:?}", a);
    //Requires template specialisation.
    a.apply_possible(settings2)?;

    println!("possible: {:?}", a);
    a.apply(module::setting_list!(NativeA(55)))?;


    println!("{:?}", a);

    Ok(())
}

pub fn completely_cross() -> plating::PlatingResult<module::CocoaWidget> {
    let a = module::CocoaWidget::new(module::setting_list!())?;

    Ok(a)
}

pub fn main() -> plating::PlatingResult<()> {
    completely_native()?;
    completely_cross()?;

    Ok(())
}
