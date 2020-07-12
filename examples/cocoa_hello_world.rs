/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use plating::prelude::*;
use plating::widgets::cocoa::{
    CocoaMenu, CocoaMenuItem, CocoaMenuItemParameters, CocoaMenuParameters, CocoaRoot,
    CocoaRootParameters, CocoaWindow, CocoaWindowParameters,
};
use plating::widgets::menu::MenuChildren;
use plating::widgets::root::{NativeRoot, RootParameters};
use plating::widgets::window::{MainMenuChildren, WindowParameters};
use plating::widgets::{default_system, System};
use plating::PlatingResult;

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::info;

///////////////////
/// begin
///////////////////
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use cocoa::appkit::NSWindow;
use cocoa::{
    base::{id, nil, selector, NO},
    foundation::NSSize,
};
use objc::declare::MethodImplementation;
use objc::runtime::{self, Class, Imp, Object, Protocol, Sel, BOOL};
use objc::{Encode, EncodeArguments, Encoding, Message};
use std::ffi::CString;
use std::fmt::Display;

extern "C" fn windowWillResize(_: &Object, _: Sel, to: NSSize) -> NSSize {
    log::info!("new size: ({},{})", to.width, to.height);
    to
}

extern "C" fn window_resizing(_: &Object, _: Sel, _: id) {
    log::warn!("resizing called!");
}

extern "C" fn window_entering_fullscreen(_: &Object, _: Sel, _: id) {
    log::warn!("fullscreen called!");
}

extern "C" fn window_entering_fullscreen2(obj: &Object, _: Sel, _: id) {
    log::warn!("fullscreen2 called! {}", plating::uuid! {});
}

/*
//copied from objc
fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{}{}{}",
        ret, <*mut Object>::ENCODING, Sel::ENCODING);
    for enc in args {
        use std::fmt::Write;
        write!(&mut types, "{}", enc).unwrap();
    }
    CString::new(types).unwrap()
}

// copied and adjusted from objc
pub unsafe fn add_method<F>(class: *mut Class, sel: Sel, func: F)
            where F: MethodImplementation<Callee=Object> {
        let encs = F::Args::ENCODINGS;
        let sel_args = sel.name().chars().filter(|&c| c == ':').count();
        assert!(sel_args == encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args, encs.len(),
        );

        let types = method_type_encoding(&F::Ret::ENCODING, encs);
        let success = runtime::class_addMethod(class, sel, func.imp(),
            types.as_ptr());
        assert!(success != NO, "Failed to add method {:?}", sel);
    }*/

///////////////////
/// end
/// ///////////////

fn main() -> PlatingResult<()> {
    simple_logger::init().unwrap(); //setting up logging

    warn!("Starting up");

    let mut x = CocoaRoot::new(CocoaRootParameters::default()).unwrap();
    let mut window = CocoaWindow::new(CocoaWindowParameters {
        ..WindowParameters {
            title: Some(String::from("yay")),
            resizable: Some(true),
            closable: Some(true),
            miniaturizable: Some(true),
            ..Default::default()
        }
        .into()
    })
    .unwrap();

    //////begin
    unsafe {
        use cocoa::appkit::NSWindow;
        use objc::declare::ClassDecl;
        use objc::runtime::class_getName;

        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MyWindowDelegate", superclass).unwrap();

        decl.add_method(
            sel!(windowWillResize:),
            windowWillResize as extern "C" fn(&Object, Sel, NSSize) -> NSSize,
        );

        decl.add_method(
            sel!(windowWillEnterFullScreen:),
            window_entering_fullscreen as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidEnterFullScreen:),
            window_entering_fullscreen as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidResize:),
            window_resizing as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidEndLiveResize:),
            window_resizing as extern "C" fn(&Object, Sel, id),
        );

        let mut delegate_class = decl.register();
        let delegate_object: *mut Object = msg_send![delegate_class, new];

        window.native().setDelegate_(delegate_object);
        log::info!("Delegated!");

        {
            let name = class_getName(delegate_class);
            let superclass = class!(NSObject);
            let mut decl = ClassDecl::new("MyWindowDelegate2", delegate_class).unwrap();

            decl.add_method(
                sel!(windowWillEnterFullScreen:),
                window_entering_fullscreen2 as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidEnterFullScreen:),
                window_entering_fullscreen2 as extern "C" fn(&Object, Sel, id),
            );

            let mut delegate_class = decl.register();
            let delegate_object: *mut Object = msg_send![delegate_class, new];

            window.native().setDelegate_(delegate_object);
        }
    }

    ////// end

    let mut menu = CocoaMenu::new(CocoaMenuParameters {
        title: Some(String::from("Process")),
        ..Default::default()
    })
    .unwrap();

    let edit = CocoaMenu::new(CocoaMenuParameters {
        title: Some(String::from("Edit")),
        ..Default::default()
    })
    .unwrap();

    let process_item1 = CocoaMenuItem::new(CocoaMenuItemParameters {
        title: Some(String::from("Yay")),
        ..Default::default()
    })
    .unwrap();
    let process_item2 = CocoaMenuItem::new(CocoaMenuItemParameters {
        title: Some(String::from("it works :)")),
        ..Default::default()
    })
    .unwrap();

    Outlet::<MenuChildren>::push_child(&mut menu, process_item1).unwrap();
    Outlet::<MenuChildren>::push_child(&mut menu, process_item2).unwrap();

    Outlet::<MainMenuChildren>::push_child(&mut window, menu)?;

    Outlet::<MainMenuChildren>::push_child(&mut window, edit)?;

    x.push_child(window)?;

    //#[allow(unused_variables)]
    //let gen = create_generic();
    //#[allow(unused_variables)]
    //let gen_nat = gen_nat();
    //#[allow(unused_variables)]
    //let nat_gen = nat_gen();

    x.run().map_err(|err| err.into())

    /*
    let mut x = Root::new(RootParameters::default()).unwrap();
    let child = Window::new(WindowParameters::default()).unwrap();
    x.add_child(child).unwrap();

    x.run().map_err(|err| err.into())*/
}

fn create_generic() -> <default_system as System>::RootType {
    let mut x = <default_system as System>::RootType::new(RootParameters::default()).unwrap();
    /*let child = Window::new(WindowParameters::default()).unwrap();
    x.add_child(child).unwrap();*/

    x
}

fn gen_nat() -> <default_system as System>::RootType {
    let mut x = <default_system as System>::RootType::new(RootParameters::default()).unwrap();
    let child1 = CocoaWindow::new(CocoaWindowParameters::default()).unwrap();
    let child2 = CocoaWindow::new(WindowParameters::default()).unwrap();
    x.push_child(child1).unwrap();
    x.push_child(child2).unwrap();
    x
}

fn nat_gen() -> CocoaRoot {
    let mut x = CocoaRoot::new(CocoaRootParameters::default()).unwrap();
    /*let child = Window::new(WindowParameters::default()).unwrap();
    x.add_child(child).unwrap();*/
    x
}
