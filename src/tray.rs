use crate::config;
// Import the required dependencies.
use std::env;

use std::path::Path;
use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use gtk::{prelude::*, MenuItem};
use open;

// -----------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn toggle_sensitivity (widget: &gtk::Widget) {
    widget.set_sensitive(!widget.is_sensitive());
}

fn tray_menu_item_clicked(item: &MenuItem) {
    println!("{} clicked!", item.label().unwrap());
}

fn tray_menu_item_open_webbrowser(_item: &MenuItem, url: &str) {
    let _ = open::that(url);
}

#[allow(dead_code)]
fn tray_menu_append_submenu (parent: &gtk::MenuItem) {
    let menu = gtk::Menu::new();

    let mi = gtk::MenuItem::with_label("Sub 1");
    mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    let prev_mi = mi;
    let mi = gtk::MenuItem::with_label("Sub 2");
    
    mi.connect_activate(glib::clone!(@weak prev_mi => move |_| {
        toggle_sensitivity(&prev_mi.upcast::<gtk::Widget>());
    }));

    menu.append(&mi);

    let mi = gtk::MenuItem::with_label("Sub 3");
    mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    menu.show_all();

    parent.set_submenu(Some(&menu));
}

fn tray_menu_append_about_submenu (parent: &gtk::MenuItem ,config_file: &str, gotify_url: &str,gotify_token: &str,ntfy_url: &str,ntfy_topics: &str) {
    let menu = gtk::Menu::new();

    let mi = gtk::MenuItem::with_label(format!("pulpo v.1.0\n(C) 2024 - Fernando Seoane Gil\nConfig file:\t\t{}\n-----------\nGotify url:\t\t{}\nGotify token:\t{}\nNtfy url:\t\t{}\nNtfy topics:\t\t{}",config_file,gotify_url,gotify_token,ntfy_url,ntfy_topics).as_str());
    //mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    menu.show_all();

    parent.set_submenu(Some(&menu));
}


pub fn build_tray_menu(config_file: &str, configdata: config::ConfigData){

    // Ref: https://github.com/rehar/appindicator3/blob/fcf1e0269065c81a4169e0a39d1cbfd0360c50d5/examples/simple_client.rs

    // Set your application name and icon
    let app_name: &str = "pulpo";
    let icon_path= Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");//"/home/efe/Dev/RustLearning/read_config_from_toml_file/resources"; //"notification.png";
    let tray_icon= configdata.config.tray_icon.as_str(); 
    let gotify_url = configdata.gotify.gotify_url;
    let gotify_token = configdata.gotify.gotify_client_token;
    let ntfy_url = configdata.ntfy.ntfy_url;
    let ntfy_topics = configdata.ntfy.ntfy_topics;

    let got_url = gotify_url.clone();
    let nfy_url = ntfy_url.clone();

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
    menu_item.connect_activate(move |item|{
        tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(), got_url.as_str())
    });
    menu.append(&menu_item);

    let menu_item = gtk::MenuItem::with_label("Open Ntfy");
    menu_item.connect_activate(move |item|{
        tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(), nfy_url.as_str())
    });
    menu.append(&menu_item);

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);
    
    let menu_item = gtk::MenuItem::with_label("About");
    // menu_item.connect_activate(|menu_item|{
    //     tray_menu_item_clicked( menu_item.upcast_ref::<gtk::MenuItem>())
    // });
    tray_menu_append_about_submenu(&menu_item,config_file,gotify_url.as_str(),gotify_token.as_str(),ntfy_url.as_str(),ntfy_topics.as_str());
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
        .icon(tray_icon, "pulpo")
        .attention_icon("notification.att.png", "pulpo attention")
        .status(IndicatorStatus::Active)
        .build();

    // Run the GTK main loop
    gtk::main();

}
