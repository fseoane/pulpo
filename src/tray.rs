

// Import the required dependencies.
use std::env;

use std::path::Path;
use appindicator3::{prelude::*, IndicatorBuilder, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use gtk::{prelude::*, MenuItem};

// use tray_icon::{
//     menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
//     TrayIconBuilder, TrayIconEvent,
// };
// use winit::event_loop::{ControlFlow, EventLoopBuilder};

// pub fn build_tray_menu(icon_filename: &str) {
//     let path = format!("{}/resources/{}",env!("CARGO_MANIFEST_DIR"),icon_filename);
//     let icon = load_icon(std::path::Path::new(&path));

//     // Since winit doesn't use gtk on Linux, and we need gtk for
//     // the tray icon to show up, we need to spawn a thread
//     // where we initialize gtk and create the tray_icon
//     #[cfg(target_os = "linux")]
//     std::thread::spawn(|| {
//         use tray_icon::menu::Menu;

//         gtk::init().unwrap();
//         let _tray_icon = TrayIconBuilder::new()
//             .with_menu(Box::new(Menu::new()))
//             .with_icon(icon)
//             .build()
//             .unwrap();

//         gtk::main();
//     });

//     let event_loop = EventLoopBuilder::new().build().unwrap();

//     //let menu_channel = MenuEvent::receiver();
//     let tray_channel = TrayIconEvent::receiver();

//     event_loop.run(move |_event, event_loop| {
//         event_loop.set_control_flow(ControlFlow::Poll);

//         if let Ok(event) = tray_channel.try_recv() {
//             println!("{event:?}");
//         }
//     });
// }

// fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
//     let (icon_rgba, icon_width, icon_height) = {
//         let image = image::open(path)
//             .expect("Failed to open icon path")
//             .into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         (rgba, width, height)
//     };
//     tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
// }

// -----------------------------------------------------------------------------------------------

pub fn tray_menu_item_clicked(item: &MenuItem) {
    println!("{} clicked!", item.label().unwrap());
}

pub fn tray_menu_append_submenu (parent: &gtk::MenuItem) {
    let menu = gtk::Menu::new();

    let mi = gtk::MenuItem::with_label("Sub 1");
    mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    let prev_mi = mi;
    let mi = gtk::MenuItem::with_label("Sub 2");
    
    // mi.connect_activate(glib::clone!(@weak prev_mi => move |_| {
    //     toggle_sensitivity(&prev_mi.upcast::<gtk::Widget>());
    // }));

    menu.append(&mi);

    let mi = gtk::MenuItem::with_label("Sub 3");
    mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    menu.show_all();

    parent.set_submenu(Some(&menu));
}

pub fn build_tray_menu(icon: &str){

    // Ref: https://github.com/rehar/appindicator3/blob/fcf1e0269065c81a4169e0a39d1cbfd0360c50d5/examples/simple_client.rs

    // Set your application name and icon
    let app_name: &str = "pulpo";
    let icon_path= Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");//"/home/efe/Dev/RustLearning/read_config_from_toml_file/resources"; //"notification.png";




    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new GTK menu
    let menu = gtk::Menu::new();

    // Create a menu items
    let menu_item = gtk::CheckMenuItem::with_label("Silent mode");
    menu_item.connect_activate(|item| {
        tray_menu_item_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);
    menu_item.show();

    let menu_item = gtk::CheckMenuItem::with_label("Do not disturb");
    menu_item.connect_activate(|item| {
        tray_menu_item_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);
    menu_item.show();

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);

    let menu_item = gtk::MenuItem::with_label("Open Gotify");
    menu_item.connect_activate(|menu_item|{
        tray_menu_item_clicked( menu_item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);

    let menu_item = gtk::MenuItem::with_label("Open Ntfy");
    menu_item.connect_activate(|menu_item|{
        tray_menu_item_clicked( menu_item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);
    
    let menu_item = gtk::MenuItem::with_label("About");
    menu_item.connect_activate(|menu_item|{
        tray_menu_item_clicked( menu_item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);

    let menu_item = gtk::MenuItem::with_label("Quit");
    menu_item.connect_activate(|_| {
        gtk::main_quit();
        std::process::exit(1);
    });
    menu.append(&menu_item);

    // Show all menu items
    menu.show_all();

    // Create a new AppIndicator
    let _indicator = Indicator::builder("pulpo")
        .title(app_name)
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&menu)
        .icon_theme_path(icon_path.to_str().unwrap())
        //.icon("notification.png", "pulpo")
        .icon(icon , "pulpo")
        .attention_icon("notification.att.png", "pulpo attention")
        .status(IndicatorStatus::Active)
        .build();

    // Run the GTK main loop
    gtk::main();

}
