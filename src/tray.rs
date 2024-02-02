

// Import the required dependencies.
use std::env;


use std::path::Path;
use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
//use gtk::prelude::*;
use gtk::{prelude::*, MenuItem};


fn tray_menu_item_clicked(item: &MenuItem) {
    println!("{} clicked!", item.label().unwrap());
}

fn tray_menu_append_submenu (parent: &gtk::MenuItem) {
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

fn build_tray_menu(){

    // Ref: https://github.com/rehar/appindicator3/blob/fcf1e0269065c81a4169e0a39d1cbfd0360c50d5/examples/simple_client.rs

    // Set your application name and icon
    let app_name: &str = "pulp0o";
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
    });
    menu.append(&menu_item);

    // Show all menu items
    menu.show_all();

    // Create a new AppIndicator
    let _indicator = Indicator::builder("Example")
        .title(app_name)
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&menu)
        //.icon_theme_path("/home/efe/Dev/RustLearning/read_config_from_toml_file/resources")
        .icon_theme_path(icon_path.to_str().unwrap())
        .icon("notification", "notification")
        .attention_icon("notification.att", "notification attention")
        .status(IndicatorStatus::Active)
        .build();

    // Run the GTK main loop
    gtk::main();


}


// fn main() {
//     // Reading command line arguments
//     let args: Vec<String> = env::args().collect();

//     let mut option: &str = "";
//     let mut parameter: &str  = "";
//     let config_option: &str = "-c";
//     let filename: &str;
    
//     if args.len()>1{
//         option = &args[1];
//     };
//     if args.len()>2{
//         parameter = &args[2];
//     } ;
//     if args.len()>2 && option.eq(config_option){
//         filename = parameter;
//     } else {
//         filename = "rNotify.conf";
//     };

//     let configdata: ConfigData = read_config(filename);

//     // Print out the values to `stdout`.
//     println!("config/tray_icon:           {}", configdata.config.tray_icon); 
//     println!("gotify/gotify_url:          {}", configdata.gotify.gotify_url);
// 	println!("gotify/gotify_client_token: {}", configdata.gotify.gotify_client_token);
// 	println!("gotify/gotify_sound:        {}", configdata.gotify.gotify_sound);
// 	println!("ntfy/ntfy_url:              {}", configdata.ntfy.ntfy_url);
// 	println!("ntfy/ntfy_topics:           {}", configdata.ntfy.ntfy_topics);
// 	println!("ntfy/ntfy_sound:            {}", configdata.ntfy.ntfy_sound);


//     let icon_filename = format!("{}{}","/home/efe/Dev/RustLearning/read_config_from_toml_file/resources/",configdata.config.tray_icon);
//     println!("{}",icon_filename);

//     env::set_var("TRAY_ICON_NAME", configdata.config.tray_icon);
//     env::set_var("GOTIFY_URL", configdata.gotify.gotify_url);
//     env::set_var("GOTIFY_CLIENT_TOKEN", configdata.gotify.gotify_client_token);

//     //build_tray_icon(concat!("/resources/",configdata.config.tray_icon));
//     build_tray_menu();

//     println!("{}", "arrived");

//     // loop{
//     //     println!("in the loop");
//     // }


// }