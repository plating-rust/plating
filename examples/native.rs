/*
 * Copyright (2020) by Marcel Lambert.
 * This project is dual licensed under either MIT or Apache-2.0.
 */

 #[macro_use]
extern crate log;
extern crate simple_logger;
extern crate libc;

use objc::*;
use cocoa::base::{selector, nil, NO, YES, BOOL, id};
use cocoa::foundation::{NSRect, NSPoint, NSSize,
    NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSAutoresizingMaskOptions, NSView,
    NSViewWidthSizable, NSViewHeightSizable,NSEventModifierFlags,
    NSWindowStyleMask, NSApp, NSWindowCollectionBehavior,
 NSWindowDepth,NSRightArrowFunctionKey, NSLeftArrowFunctionKey, 
NSApplication, NSApplicationActivationPolicyRegular,
NSWindow, NSBackingStoreBuffered, NSColorSpace,
NSMenu, NSMenuItem, NSRunningApplication,
NSApplicationActivateIgnoringOtherApps};

pub fn assert_main_thread() {
    unsafe {
        let is_main_thread: BOOL = msg_send!(class!(NSThread), isMainThread);
        assert_eq!(is_main_thread, YES);
    }
}

pub fn make_ns_string(s: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}
pub fn string_with_characters(character: libc::c_ushort) -> id {
    unsafe { 
        msg_send![class!(NSString), stringWithCharacters:&character length:1]
    }
}

fn main() {
    simple_logger::init().unwrap(); //setting up logging

    info!("Creating window");


    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        let edit_menu_item = NSMenuItem::alloc(nil);
        edit_menu_item.autorelease();
        menubar.addItem_(edit_menu_item);
        
        let edit_menu = NSMenu::alloc(nil);
        edit_menu.initWithTitle_(make_ns_string("Bearbeiten"));
        edit_menu.autorelease();


        let undo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_ns_string("Undo"),
            sel!(undo:),
            make_ns_string("z"),
        );
        edit_menu.addItem_(undo);
        edit_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        
        let redo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_ns_string("Redo"),
            sel!(redo:),
            make_ns_string("z"),
        );
        redo.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSShiftKeyMask | 
            NSEventModifierFlags::NSCommandKeyMask
        );
        edit_menu.addItem_(redo);

        edit_menu_item.setSubmenu_(edit_menu);

        // create Window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(200., 200.)),
                                                          NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO)
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        let title = NSString::alloc(nil).init_str("Hello World!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.setMainMenu_(menubar);
        app.run();
    }

/*
    
    let rect = NSRect::new(
        NSPoint::new(0., 0.),
        NSSize::new(300., 300.),  //TODO: get from settings!
    );

    let style_mask = //NSWindowStyleMask::NSWindowStyleMaskTitled |
                     //NSWindowStyleMask::NSWindowStyleMaskClosable
                     NSWindowStyleMask::NSClosableWindowMask
                   | NSWindowStyleMask::NSMiniaturizableWindowMask
                   | NSWindowStyleMask::NSResizableWindowMask
                   | NSWindowStyleMask::NSUnifiedTitleAndToolbarWindowMask
                   | NSWindowStyleMask::NSTitledWindowMask;

    let options: NSAutoresizingMaskOptions = NSViewWidthSizable | NSViewHeightSizable;
    

    assert_main_thread();

    unsafe {

        ///////// Main Menu
        let menubar = NSMenu::alloc(nil);
        menubar.autorelease();
        


        let app_menu_item = NSMenuItem::alloc(nil);
        app_menu_item.autorelease();
        menubar.addItem_(app_menu_item);

        let file_menu_item = NSMenuItem::alloc(nil);
        file_menu_item.autorelease();
        menubar.addItem_(file_menu_item);

        let edit_menu_item = NSMenuItem::alloc(nil);
        edit_menu_item.autorelease();
        menubar.addItem_(edit_menu_item);

        let view_menu_item = NSMenuItem::alloc(nil);
        view_menu_item.autorelease();
        menubar.addItem_(view_menu_item);

        let window_menu_item = NSMenuItem::alloc(nil);
        window_menu_item.autorelease();
        menubar.addItem_(window_menu_item);

        let help_menu_item = NSMenuItem::alloc(nil);
        help_menu_item.autorelease();
        menubar.addItem_(help_menu_item);
        

        let app_menu = NSMenu::alloc(nil);
        app_menu.initWithTitle_(make_ns_string("Calpipe"));
        app_menu.autorelease();

        let file_menu = NSMenu::alloc(nil);
        file_menu.initWithTitle_(make_ns_string("File"));
        file_menu.autorelease();

        let edit_menu = NSMenu::alloc(nil);
        edit_menu.initWithTitle_(make_nsstring("Bearbeiten"));
        edit_menu.autorelease();

        let view_menu = NSMenu::alloc(nil);
        view_menu.initWithTitle_(make_nsstring("View"));
        view_menu.autorelease();

        let window_menu = NSMenu::alloc(nil);
        window_menu.initWithTitle_(make_nsstring("Window"));
        window_menu.autorelease();

        let help_menu = NSMenu::alloc(nil);
        help_menu.initWithTitle_(make_nsstring("Help"));
        help_menu.autorelease();

        

        let about = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("About calpipe"),
            sel!(orderFrontStandardAboutPanel:),
            make_nsstring(""),
        );
        app_menu.addItem_(about);

        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let preferences = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Preferences..."),
            sel!(preferences:),
            make_nsstring(","),
        );
        app_menu.addItem_(preferences);

        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        //Services -> submenu
        //TODO:
        
        //let services = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        //    make_nsstring("Services"),
        //    sel!(services:),
        //    make_nsstring(""),
        //);

        //let services_menu = NSMenu::alloc(nil);
        //let i : id = msg_send![NSMenu(), services];
        //let _ : BOOL = msg_send![services_menu, init:make_nsstring("Services") identifier:i];
        
        //services_menu.initWithTitle_(make_nsstring("Calpipe"));
        //services_menu.autorelease();
        //services.setSubmenu_(services_menu);
        //NSApp().setWindowsMenu_(services_menu);

        //app_menu.addItem_(services);
        

        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let hide = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Hide calpipe"),
            sel!(hide:),
            make_nsstring("h"),
        );
        app_menu.addItem_(hide);

        let hide_others = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Hide others"),
            sel!(hideOtherApplications:),
            make_nsstring("h"),
        );
        hide_others.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSAlternateKeyMask | 
            NSEventModifierFlags::NSCommandKeyMask
        );
        app_menu.addItem_(hide_others);

        let show_all = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Show All"),
            sel!(unhideAllApplications:),
            make_nsstring(""),
        );
        app_menu.addItem_(show_all);

        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let quit_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Quit calpipe"),
            sel!(terminate:),
            make_nsstring("q"),
        );
        app_menu.addItem_(quit_menu_item);



        //////////
        // File Menu
        ////////// 
        let new_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("New"),
            sel!(new:),
            make_nsstring("n"),
        );
        file_menu.addItem_(new_menu_item);

        let new_event_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("New Event"),
            sel!(new_event:),
            make_nsstring(""),
        );
        file_menu.addItem_(new_event_menu_item);

        let new_todo_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("New Todo"),
            sel!(new_todo:),
            make_nsstring(""),
        );
        file_menu.addItem_(new_todo_menu_item);

        let new_todo_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Sync now"),
            sel!(new_todo:),
            make_nsstring(""),
        );

        file_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let new_todo_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Import"),
            sel!(new_todo:),
            make_nsstring(""),
        );
        file_menu.addItem_(new_todo_menu_item);
        let new_todo_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Export"),
            sel!(new_todo:),
            make_nsstring(""),
        );
        file_menu.addItem_(new_todo_menu_item);
        let new_todo_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Schliessen"),
            sel!(performClose:),
            make_nsstring("w"),
        );
        file_menu.addItem_(new_todo_menu_item);

        file_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());



        file_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        
        let print_menu_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Print"),
            sel!(print:),
            make_nsstring("p"),
        );
        file_menu.addItem_(print_menu_item);
        
        //////////
        // Edit Menu
        //////////
        
        let undo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Undo"),
            sel!(undo:),
            make_nsstring("z"),
        );
        edit_menu.addItem_(undo);

        let redo = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Redo"),
            sel!(redo:),
            make_nsstring("z"),
        );
        redo.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSShiftKeyMask | 
            NSEventModifierFlags::NSCommandKeyMask
        );
        edit_menu.addItem_(redo);

        edit_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        
        let cut = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Cut"),
            sel!(cut:),
            make_nsstring("x"),
        );
        edit_menu.addItem_(cut);

        let copy = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Copy"),
            sel!(copy:),
            make_nsstring("c"),
        );
        edit_menu.addItem_(copy);

        let paste = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Paste"),
            sel!(paste:),
            make_nsstring("v"),
        );
        edit_menu.addItem_(paste);

        let duplicate = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Duplicate"),
            sel!(duplicate:),
            make_nsstring("d"),
        );
        edit_menu.addItem_(duplicate);

        let select_all = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Select all"),
            sel!(delete:),
            make_nsstring("a"),
        );
        edit_menu.addItem_(select_all);


        //edit_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

       
        ////////
        // View Menu
        ////////
        
        
        ///Show/Hide Tab Bar
        ///Show All Tabs/Exit Tab Overview	
        
        
        let day = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Tagesansicht"),
            sel!(day:),
            make_nsstring("1"),
        );
        view_menu.addItem_(day);

        let week = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Wochenansicht"),
            sel!(week:),
            make_nsstring("2"),
        );
        view_menu.addItem_(week);

        let month = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Monatsansicht"),
            sel!(month:),
            make_nsstring("3"),
        );
        view_menu.addItem_(month);

        let year = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Jahresansicht"),
            sel!(year:),
            make_nsstring("4"),
        );
        view_menu.addItem_(year);

        let list = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Liste"),
            sel!(list:),
            make_nsstring("5"),
        );
        view_menu.addItem_(list);
        
        view_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        
        let find = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Search"),
            sel!(find:),
            make_nsstring("s"),
        );
        view_menu.addItem_(find);

        let toggle_filter = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Show Filter"),
            sel!(toggle_filter:),
            make_nsstring("f"),
        );
        toggle_filter.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSShiftKeyMask | 
            NSEventModifierFlags::NSCommandKeyMask
        );
        view_menu.addItem_(toggle_filter);

        view_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let find = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Show Sources"),
            sel!(find:),
            make_nsstring(""),
        );
        view_menu.addItem_(find);

        let find = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Show Details"),
            sel!(find:),
            make_nsstring(""),
        );
        view_menu.addItem_(find);

        view_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());


        let previous = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Previous"),
            sel!(previous:),
            string_with_characters(NSLeftArrowFunctionKey),
        );
        view_menu.addItem_(previous);

        let next = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Next"),
            sel!(next:),
            string_with_characters(NSRightArrowFunctionKey),
        );
        view_menu.addItem_(next);

        let today = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Today"),
            sel!(next:),
            make_nsstring("t"),
        );
        view_menu.addItem_(today);

        view_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        
        //////////////
        // windows
        //////////////
        let minimize = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Minimize"),
            sel!(performMiniaturize:),
            make_nsstring("m"),
        );
        window_menu.addItem_(minimize);

        let minimize_all = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Minimize All"),
            sel!(performMiniaturizeAll:),
            make_nsstring("m"),
        );
        minimize_all.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSAlternateKeyMask | 
            NSEventModifierFlags::NSCommandKeyMask
        );
        window_menu.addItem_(minimize_all);

        window_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let previous_tab = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Previous Tab"),
            sel!(todo:),
            make_nsstring("\t"),
        );
        previous_tab.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSControlKeyMask | 
            NSEventModifierFlags::NSShiftKeyMask
        );
        window_menu.addItem_(previous_tab);

        //show next tab
        let next_tab = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Next Tab"),
            sel!(todo:),
            make_nsstring("\t"),
        );
        next_tab.setKeyEquivalentModifierMask_(
            NSEventModifierFlags::NSControlKeyMask
        );
        window_menu.addItem_(next_tab);

        ///move tab to new window
        ///Should not work for 'main' tab
        let tab_to_new_window = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Move Tab to New Window"),
            sel!(moveTabToNewWindow:),
            make_nsstring(""),
        );
        window_menu.addItem_(tab_to_new_window);

        ///merge all window
        let merge_all_window = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Merge all Windows"),
            sel!(mergeAllWindows:),
            make_nsstring(""),
        );
        window_menu.addItem_(merge_all_window); 

        window_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let all_front = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Bring all to Front"),
            sel!(arrangeInFront:),
            make_nsstring(""),
        );
        window_menu.addItem_(all_front);

        view_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        //////////////////
        // Help
        //////////////////
        let help = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Calpipe Help"),
            sel!(showHelp:),
            make_nsstring("?"),
        );
        help_menu.addItem_(help);

        let keyboard_shortcuts = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Keyboard Shortcuts"),
            sel!(showKeyboardShortcuts:),
            make_nsstring(""),
        );
        help_menu.addItem_(keyboard_shortcuts);
        
        help_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        let links_entry = NSMenuItem::alloc(nil).autorelease();
        let link_menu = NSMenu::alloc(nil);
        link_menu.initWithTitle_(make_nsstring("Links"));
        link_menu.autorelease();
        links_entry.setSubmenu_(link_menu);
        help_menu.addItem_(links_entry);

        //////////////////
        // Links
        //////////////////
        
        ///website
        let website = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("www.WEBSITE_TODO.de"),
            sel!(open_website:),
            make_nsstring(""),
        );
        link_menu.addItem_(website);

        /// Documentation
        /// Code
        /// Issue Tracker
        link_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());

        ///Local Installation
        let website = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            make_nsstring("Open Installation folder"),
            sel!(installation_folder:),
            make_nsstring(""),
        );
        link_menu.addItem_(website);
        /// Licence


        app_menu_item.setSubmenu_(app_menu);
        edit_menu_item.setSubmenu_(edit_menu);
        view_menu_item.setSubmenu_(view_menu);
        window_menu_item.setSubmenu_(window_menu);
        help_menu_item.setSubmenu_(help_menu);

        
        
        ////////// Window
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            rect,
            style_mask, //TODO:
            NSBackingStoreBuffered,
            NO,
        );
        window.autorelease();
        info!("window created");

        window.cascadeTopLeftFromPoint_(NSPoint::new(20.0, 20.0)); //TODO: get from settings
        
        //window.setTitle_(make_nsstring("Test"));

        //TODO: window.addTitlebarAccessoryViewController_();
        //let titlebarAccessory: id = msg_send![class!(NSTitlebarAccessoryViewController), alloc];
        //titlebarAccessory.autorelease();


        //let layout = NSLayoutAttribute::NSLayoutAttributeLeft;
        //lOSXApplication::setlayoutAttribute_(titlebarAccessory, layout);
        //let res: BOOL = msg_send![titlebarAccessory, setLayoutAttribute:layout.bits];
        //if (res == NO) {

        //}
        //let _ : BOOL = msg_send![window, addTitlebarAccessoryViewController:titlebarAccessory];

        window.makeKeyAndOrderFront_(nil);
        //window.setAcceptsMouseMovedEvents_(YES);

        //let (view, idle_queue) = make_view(self.handler.expect("view"));
        //let view: id = msg_send![VIEW_CLASS.0, new];
        
        //TODO: set to window, not app in general
        //NSApp().setMainMenu_(menubar);
       // window.setMainMenu(menubar);

        //let content_view = window.contentView();
        //let frame = NSView::frame(content_view);
        //let view = NSView::alloc(nil).initWithFrame_(frame);
        //view.setAutoresizingMask_(options);

        //let () = msg_send![window, setDelegate: view];

        //NSApp().activateIgnoringOtherApps_(YES);

        //view.autorelease();
        info!("run");
        NSApp().run();
    }
    info!("window creation done!");*/
}