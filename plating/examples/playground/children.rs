/*
 * Copyright (2021) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

pub mod module {
    pub use plating::utils::children::{children_list, ChildrenList};

    /*pub trait HasChildren<CHILDREN>
    where
        CHILDREN: ChildrenList,
    {
        fn children(&self) -> &CHILDREN;
        fn mut_children(&self) -> &CHILDREN;
    }*/


    pub trait CocoaChildOf<T>: Sized {
        fn connect(&self, parent: &T);
        fn disconnect(&self);
    }

    impl<T, Y> CocoaChildOf<Y> for T
    where
        T: ChildOf<Y>,
    {
        fn connect(&self, parent: &Y) {
            <Self as ChildOf<Y>>::connect(self, parent);
        }

        fn disconnect(&self) {
            <Self as ChildOf<Y>>::disconnect(self);
        }
    }
    pub trait ChildOf<T>: Sized {
        fn connect(&self, parent: &T);
        fn disconnect(&self);
    }

    impl<T> ChildOf<T> for () {
        fn connect(&self, _parent: &T) {}

        fn disconnect(&self) {}
    }

    impl<T, Head, Tail> ChildOf<T> for (Head, Tail)
    where
        Head: ChildOf<T>,
        Tail: ChildOf<T> + ChildrenList,
    {
        fn connect(&self, parent: &T) {
            self.0.connect(parent);
            self.1.connect(parent);
        }

        fn disconnect(&self) {
            self.0.disconnect();
            self.1.disconnect();
        }
    }


    //////////////////////////////////////////////////////////////////////
    // Parent Widget
    //////////////////////////////////////////////////////////////////////
    #[derive(Debug)]
    pub struct CocoaParentWidget<CHILDREN>
    where
        CHILDREN: ChildrenList,
    {
        children: CHILDREN,
    }

    pub trait CocoaParent<CHILDREN>
    where
        CHILDREN: ChildrenList + CocoaChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self;
    }

    impl<CHILDREN> CocoaParent<CHILDREN> for CocoaParentWidget<CHILDREN>
    where
        CHILDREN: ChildrenList + CocoaChildOf<Self>,
    {
        fn new(children: CHILDREN) -> Self {
            let result = CocoaParentWidget::<CHILDREN> { children };
            result.children.connect(&result);

            result
        }
    }

    pub trait CrossParent<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self;
    }

    impl<CHILDREN> CrossParent<CHILDREN> for CocoaParentWidget<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self {
            Self { children }
        }
    }

    //////////////////////////////////////////////////////////////////////
    // Cocoa Child
    //////////////////////////////////////////////////////////////////////
    #[derive(Debug)]
    pub struct CocoaChildWidget<CHILDREN>
    where
        CHILDREN: ChildrenList,
        Self: Sized,
    {
        children: CHILDREN,
    }

    /*
    impl<CHILDREN> HasChildren<CHILDREN> for CocoaChildWidget<CHILDREN>
    where
        CHILDREN: ChildrenList,
        Self: Sized,
    {
        fn children(&self) -> &CHILDREN {
            &self.children
        }

        fn mut_children(&self) -> &CHILDREN {
            &self.children
        }
    }*/

    pub trait CrossChild<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self;
    }

    pub trait CocoaChild<CHILDREN>
    where
        CHILDREN: ChildrenList + CocoaChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self;
    }


    impl<ChildrenP, ChildrenC> ChildOf<CocoaParentWidget<ChildrenP>> for CocoaChildWidget<ChildrenC>
    where
        ChildrenP: ChildrenList + std::fmt::Debug,
        ChildrenC: ChildrenList + std::fmt::Debug,
    {
        fn connect(&self, parent: &CocoaParentWidget<ChildrenP>) {
            println!("Connecting {:?} to {:?}", self, parent);
        }

        fn disconnect(&self) {
            println!("Disconnecting {:?}", self);
        }
    }

    impl<ChildrenP, ChildrenC> ChildOf<CocoaChildWidget<ChildrenP>> for CocoaChildWidget<ChildrenC>
    where
        ChildrenP: ChildrenList + std::fmt::Debug,
        ChildrenC: ChildrenList + std::fmt::Debug,
    {
        fn connect(&self, parent: &CocoaChildWidget<ChildrenP>) {
            println!("Connecting {:?} to {:?}", self, parent);
        }

        fn disconnect(&self) {
            println!("Disconnecting {:?}", self);
        }
    }

    impl<CHILDREN> CocoaChild<CHILDREN> for CocoaChildWidget<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>,
    {
        fn new(children: CHILDREN) -> Self {
            Self { children }
        }
    }

    impl<CHILDREN> CrossChild<CHILDREN> for CocoaChildWidget<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>,
        Self: Sized,
    {
        fn new(children: CHILDREN) -> Self {
            Self { children }
        }
    }

    //////////////////////////////////////////////////////////////////////
    // Self Child
    //////////////////////////////////////////////////////////////////////
    /*pub struct CocoaSelfChild<'a, CHILDREN: ChildrenList + ChildOf<'a, Self>, PARENT> {
        children: CHILDREN,
        parent: Cell<Option<&'a PARENT>>,
    }

    impl<'a, CHILDREN_P, CHILDREN_C, PARENT_P, PARENT_C> ChildOf<CocoaChildWidget<'a, CHILDREN_P, PARENT_P>> for CocoaChildWidget<'a, CHILDREN_C, PARENT_C>
    where
        CHILDREN_P: ChildrenList,
        CHILDREN_C: ChildrenList + ChildOf<Self>,
    {
        fn connect(&'a mut self, parent: &'a CocoaChildWidget<'a CHILDREN_P, PARENT_P>)  {}
        fn disconnect(&'a mut self) {}
    }

    impl<CHILDREN: ChildrenList + ChildOf<Self>> CocoaParent<CHILDREN> for CocoaSelfChild<CHILDREN> {
        fn new(children: CHILDREN) -> Self {
            Self {
                children: children,
                parent: None,
            }
        }
    }

    impl<CHILDREN> CrossParent<CHILDREN> for CocoaSelfChild<CHILDREN>
    where
        CHILDREN: ChildrenList + ChildOf<Self>
    {
        fn new(children: CHILDREN) -> Self {
            Self {
                children: children,
                parent: None,
            }
        }
    }*/
    /*
    pub struct CocoaChildOnly {}

    impl<CHILDREN_P> ChildOf<CocoaParentWidget<CHILDREN_P>> for CocoaChildOnly
    where
        CHILDREN_P: ChildrenList + ChildOf<CocoaParentWidget<CHILDREN_P>>,
    {
        fn connect(&mut self, parent: &CocoaParentWidget<CHILDREN_P>)  {}
        fn disconnect(&mut self) {}
    }

    impl CocoaParent<()> for CocoaChildOnly {
        fn new(children: ()) -> Self {
            Self { }
        }
    }

    impl CrossParent<()> for CocoaChildOnly {
        fn new(children: ()) -> Self {
            Self {}
        }
    }*/
}


pub fn completely_static_native() -> plating::PlatingResult<impl module::ChildrenList> {
    let a = module::children_list!(
        <module::CocoaParentWidget<_> as module::CocoaParent<_>>::new(module::children_list!(
            <module::CocoaChildWidget<_> as module::CocoaChild<_>>::new(()),
            <module::CocoaChildWidget<_> as module::CocoaChild<_>>::new(module::children_list!(
                <module::CocoaChildWidget<_> as module::CocoaChild<_>>::new(()),
            )),
        ))
    );
    Ok(a)
}

pub fn completely_static_cross() -> plating::PlatingResult<impl module::ChildrenList> {
    use module::CrossParent;
    Ok(module::children_list!(module::CocoaParentWidget::new(
        module::children_list!(
            /*module::CocoaChildWidget::new(()
                //module::children_list!(/*module::CocoaChildOnly::new(())*/),
            ),*/
            //module::CocoaSelfChild::new(()),
            //module::CocoaChildOnly::new(()),
        )
    )))
}

pub fn completely_static_both() -> plating::PlatingResult<impl module::ChildrenList> {
    use module::CrossParent;
    Ok(module::children_list!(module::CocoaParentWidget::new(
        module::children_list!(
            //module::CocoaChild::new(()),
            //module::CocoaSelfChild::new(),
            //module::CocoaChildOnly::new(()),
            //<module::CocoaChildOnly as module::CocoaParent<_>>::new(()),
        )
    )))
}



pub fn with_dynamics() -> plating::PlatingResult<impl module::ChildrenList> {
    use module::CrossParent;
    Ok(module::children_list!(module::CocoaParentWidget::new(
        module::children_list!(
            //module::CocoaSelfChild::new(()),
            //module::CocoaChildOnly::new(()),
        )
    )))
}


pub fn main() -> plating::PlatingResult<()> {
    completely_static_native()?;
    completely_static_cross()?;
    completely_static_both()?;

    with_dynamics()?;

    Ok(())
}
