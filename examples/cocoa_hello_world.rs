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

fn main() -> PlatingResult<(), default_system> {
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
