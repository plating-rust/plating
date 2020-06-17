/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

use plating::prelude::*;
use plating::widgets::cocoa::{
    CocoaRoot, CocoaRootParameters,
    CocoaWindow, CocoaWindowParameters,
    CocoaMenu, CocoaMenuParameters, CocoaMenuItem, CocoaMenuItemParameters,
};
use plating::widgets::{default_system, MenuChildren, MainMenuChildren};
use plating::widgets::generic::{RootWidgetTrait, Root, RootParameters, Window, WindowParameters};
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
        }.into()
    }).unwrap();
    
    let mut menu = CocoaMenu::new(CocoaMenuParameters {
        title: Some(String::from("Process")),
        ..Default::default()
    }).unwrap();

    let edit = CocoaMenu::new(CocoaMenuParameters {
        title: Some(String::from("Edit")),
        ..Default::default()
    }).unwrap();


    let process_item1 = CocoaMenuItem::new(CocoaMenuItemParameters {
        title: Some(String::from("Yay")),
        ..Default::default()
    }).unwrap();
    let process_item2 = CocoaMenuItem::new(CocoaMenuItemParameters {
        title: Some(String::from("it works :)")),
        ..Default::default()
    }).unwrap();
    
    OutletAdapter::<MenuChildren>::add_child(&mut menu, process_item1);
    OutletAdapter::<MenuChildren>::add_child(&mut menu, process_item2);


    OutletAdapter::<MainMenuChildren>::add_child(&mut window, menu)?;

    
    OutletAdapter::<MainMenuChildren>::add_child(&mut window, edit)?;




    x.add_child(window)?;

    #[allow(unused_variables)]
    let gen = create_generic();
    #[allow(unused_variables)]
    let gen_nat = gen_nat();
    #[allow(unused_variables)]
    let nat_gen = nat_gen();


    x.run().map_err(|err| err.into())
}

fn create_generic() -> Root {
    let mut x = Root::new(RootParameters::default()).unwrap();
    let child = Window::new(WindowParameters::default()).unwrap();
    x.add_child(child).unwrap();

    x
}

fn gen_nat() -> Root {
    let mut x = Root::new(RootParameters::default()).unwrap();
    let child1 = CocoaWindow::new(CocoaWindowParameters::default()).unwrap();
    let child2 = CocoaWindow::new(WindowParameters::default()).unwrap();
    x.add_child(child1).unwrap();
    x.add_child(child2).unwrap();
    x
}

fn nat_gen() -> CocoaRoot {
    let mut x = CocoaRoot::new(CocoaRootParameters::default()).unwrap();
    let child = Window::new(WindowParameters::default()).unwrap();
    x.add_child(child).unwrap();
    x
}
