// Import the required dependencies.
use crate::config::{read_config,NtfyConf, ConfigData, GotifyConf};
use std::env;
use std::path::Path;
use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use gtk::{prelude::*, MenuItem};
use open;
use log::{error, info, warn};

// -----------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn toggle_sensitivity (widget: &gtk::Widget) {
    widget.set_sensitive(!widget.is_sensitive());
}


fn tray_menu_item_clicked(item: &MenuItem) {
    info!("Menu item {} clicked!", item.label().unwrap());
}


fn tray_silent_clicked(_item: &MenuItem) {
    let current = std::env::var("SILENT").unwrap();
    if current=="on"{
        env::set_var("SILENT", String::from("off"));
    } else if current=="off"{
        env::set_var("SILENT", String::from("on"));
    };
    info!("SILENT:{}",std::env::var("SILENT").unwrap());
}


fn tray_dnd_clicked(_item: &MenuItem) {
    let current = std::env::var("DND").unwrap();
    if current=="on"{
        env::set_var("DND", String::from("off"));
    } else if current=="off"{
        env::set_var("DND", String::from("on"));
    };
    info!("DND:{}",std::env::var("DND").unwrap());
}


fn tray_menu_item_open_webbrowser(_item: &MenuItem, url: &str) {
    let _ = open::that(url);
}


// #[allow(dead_code)]
// fn tray_menu_append_submenu (parent: &gtk::MenuItem) {
//     let menu = gtk::Menu::new();

//     let mi = gtk::MenuItem::with_label("Sub 1");
//     mi.connect_activate(tray_menu_item_clicked);
//     menu.append(&mi);

//     let prev_mi = mi;
//     let mi = gtk::MenuItem::with_label("Sub 2");
    
//     mi.connect_activate(glib::clone!(@weak prev_mi => move |_| {
//         toggle_sensitivity(&prev_mi.upcast::<gtk::Widget>());
//     }));

//     menu.append(&mi);

//     let mi = gtk::MenuItem::with_label("Sub 3");
//     mi.connect_activate(tray_menu_item_clicked);
//     menu.append(&mi);

//     menu.show_all();

//     parent.set_submenu(Some(&menu));
// }


fn tray_menu_append_about_submenu (
    parent: &gtk::MenuItem,
    config_file: &str, 
    gotify_url: &str,
    gotify_token: &str,
    ntfy_url: &str,
    ntfy_topics: &str) {

    let menu = gtk::Menu::new();

    let mut label: String = String::from("");

    let app_and_author_str: &str = "pulpo v.1.3\n(C) 2024 - Fernando Seoane Gil\n";
    let config_file_str: String = format!("Config file:\t\t{}\n-----------\n",config_file);
    label = format!("{}{}",app_and_author_str,config_file_str);
  
    let gotify_conf_url: &str = gotify_url;
    let gotify_conf_token: &str = gotify_token;


    let gotify_conf_str: String;
    if !String::from(gotify_conf_url).is_empty() {
        gotify_conf_str = format!("Gotify url:\t\t{}\nGotify token:\t{}\n",gotify_conf_url,gotify_conf_token);
        label = format!("{}{}",label,gotify_conf_str);
    };

    let ntfy_conf_url: &str = ntfy_url;
    let ntfy_conf_topics: &str = ntfy_topics;

    let ntfy_conf_str: String;
    if !String::from(ntfy_conf_url).is_empty(){
        ntfy_conf_str = format!("Ntfy url:\t\t{}\nNtfy topics:\t\t{}",&ntfy_conf_url,&ntfy_conf_topics);
        label = format!("{}{}",label,ntfy_conf_str);
    };
    
  
    let mi = gtk::MenuItem::with_label(label.as_str());

    //mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    menu.show_all();
    parent.set_submenu(Some(&menu));

}


pub fn build_tray_menu(config_file: &str, tray_icon: &str, gotify_url: &str, gotify_token: &str,ntfy_url: &str, ntfy_topics: &str){

    // Ref: https://github.com/rehar/appindicator3/blob/fcf1e0269065c81a4169e0a39d1cbfd0360c50d5/examples/simple_client.rs

    // Set your application name and icon
    let app_name: &str = "pulpo";
    //let icon_path= Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    let icon_path= std::env::current_dir().unwrap().join("resources/");

    let got_url = String::from(gotify_url);
    let got_url_copy = got_url.clone();     // this clone is needed to 'move´ it to the next closures of tray_menu_item_open_webbrowser
    let got_token = String::from(gotify_token);

    
    let nfy_url =String::from(ntfy_url);
    let nfy_url_copy =  nfy_url.clone();    // this clone is needed to 'move´ it to the next closures of tray_menu_item_open_webbrowser
    let nfy_topics = String::from(ntfy_topics); 

    let has_gotify_config = !String::from(got_url.clone()).is_empty();
    let has_ntfy_config = !String::from(nfy_url.clone()).is_empty();
    
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new GTK menu
    let menu = gtk::Menu::new();

    // Create a menu items
    let menu_item = gtk::CheckMenuItem::with_label("Silent mode");
    menu_item.connect_activate(|item| {
        tray_silent_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);
    menu_item.show();

    let menu_item = gtk::CheckMenuItem::with_label("Do not disturb");
    menu_item.connect_activate(|item| {
        tray_dnd_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&menu_item);
    menu_item.show();

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);
    
   
    if has_gotify_config{
        let menu_item = gtk::MenuItem::with_label("Open Gotify");
        menu_item.connect_activate( move |item |{
            tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(),got_url_copy.as_str())
        });
        menu.append(&menu_item);
    };

    if has_ntfy_config{
        let menu_item = gtk::MenuItem::with_label("Open Ntfy");
        menu_item.connect_activate( move |item |{
            tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(),nfy_url_copy.as_str())
        });
        menu.append(&menu_item);
    };

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);
    
    let menu_item = gtk::MenuItem::with_label("About");
    tray_menu_append_about_submenu(
        &menu_item,
        config_file,
        &got_url.as_str(),
        &got_token.as_str(),
        &nfy_url.as_str(),
        &nfy_topics.as_str()
    );
    //tray_menu_append_about_submenu(&menu_item,config_file,got_url,got_token,nfy_url,nfy_topics);
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
    //println!("    icon_path:          {}", icon_path.to_str().unwrap());
    //println!("    tray_icon:          {}", tray_icon);  
    let _indicator = Indicator::builder("pulpo")
        .title(app_name)
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&menu)
        .icon_theme_path(icon_path.to_str().unwrap())
        .icon(tray_icon, "pulpo")
        .attention_icon("notification.att.png", "pulpo attention")
        .status(IndicatorStatus::Active)
        .build();

    // Run the GTK main loop
    gtk::main();

}
