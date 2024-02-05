use crate::config::{read_config,NtfyConf, ConfigData, GotifyConf};
use std::borrow::Borrow;
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

fn tray_silent_clicked(_item: &MenuItem) {
    let current = std::env::var("SILENT").unwrap();
    println!("SILENT current:{}",current);
    if current=="on"{
        env::set_var("SILENT", String::from("off"));
    } else if current=="off"{
        env::set_var("SILENT", String::from("on"));
    };
    println!("SILENT after:{}",std::env::var("SILENT").unwrap());
}

fn tray_dnd_clicked(_item: &MenuItem) {
    let current = std::env::var("DND").unwrap();
    println!("DND current:{}",current);
    if current=="on"{
        env::set_var("DND", String::from("off"));
    } else if current=="off"{
        env::set_var("DND", String::from("on"));
    };
    println!("DND after:{}",std::env::var("DND").unwrap());
}

fn tray_menu_item_open_webbrowser(_item: &MenuItem, url: &str) {
    let _ = open::that(url);
}

// fn tray_menu_item_open_webbrowser(url: &str) {
//     let _ = open::that(url);
// }

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

fn tray_menu_append_about_submenu2 (
    parent: &gtk::MenuItem,
    config_file: &str, 
    gotify_url: &str,
    gotify_token: &str,
    ntfy_url: &str,
    ntfy_topics: &str) {

    let menu = gtk::Menu::new();

    let app_and_author_str: &str = "pulpo v.1.0\n(C) 2024 - Fernando Seoane Gil\n";
    let config_file_str: String = format!("Config file:\t\t{}\n-----------\n",config_file);
  
    let gotify_conf_url: &str = gotify_url.clone();
    let gotify_conf_token: &str = gotify_token.clone();

    let gotify_conf_str: String;
    if !String::from(gotify_conf_url).is_empty() {
        gotify_conf_str = format!("Gotify url:\t\t{}\nGotify token:\t{}\n",gotify_conf_url,gotify_conf_token);
    };

    let ntfy_conf_url: &str = ntfy_url.clone();
    let ntfy_conf_topics: &str = ntfy_topics.clone();

    let ntfy_conf_str: String;
    if !String::from(ntfy_conf_url).is_empty(){
        ntfy_conf_str = format!("Ntfy url:\t\t{}\nNtfy topics:\t\t{}",&ntfy_conf_url,&ntfy_conf_topics);
    };
    
    let mi = gtk::MenuItem::with_label(format!("{}{}{}{}",app_and_author_str,config_file_str,gotify_conf_str,ntfy_conf_str).as_str());

    //mi.connect_activate(tray_menu_item_clicked);
    menu.append(&mi);

    menu.show_all();
    parent.set_submenu(Some(&menu));

}


//pub fn build_tray_menu<'l>(config_file: &str, tray_icon: &str, gotify_url: &'static str, gotify_token: &'static str,ntfy_url: &'static str, ntfy_topics: &'static str){
pub fn build_tray_menu(config_file: &str, tray_icon: &str, gotify_url: &str, gotify_token: &str,ntfy_url: &str, ntfy_topics: &str){

    // Ref: https://github.com/rehar/appindicator3/blob/fcf1e0269065c81a4169e0a39d1cbfd0360c50d5/examples/simple_client.rs

    // Set your application name and icon
    let app_name: &str = "pulpo";
    let icon_path= Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let mut has_gotify_config: bool = false;
    let mut has_ntfy_config: bool = false;

    // let got_token = String::from(gotify_token);
    // let got_url = String::from(gotify_url);
    // let nfy_url =String::from(ntfy_url);
    // let nfy_topics = String::from(ntfy_topics);
    

    let got_url = gotify_url.clone();
    env::set_var("GOTIFY_URL", got_url);

    let nfy_url = ntfy_url.clone();
    env::set_var("NTFY_URL", nfy_url);

    let got_token = gotify_token;
    let nfy_topics = ntfy_topics;

    // has_gotify_config = !got_token.is_empty();
    // has_ntfy_config = !nfy_topics.is_empty();
    has_gotify_config = !String::from(got_token).is_empty();
    has_ntfy_config = !String::from(nfy_topics).is_empty();
    
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
    
   
    let menu_item = gtk::MenuItem::with_label("Open Gotify");
    menu_item.connect_activate( |item |{
        tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(),std::env::var("GOTIFY_URL").unwrap().as_str())
    });
    if has_gotify_config{
        menu.append(&menu_item);
    };

    if has_ntfy_config{
        let menu_item = gtk::MenuItem::with_label("Open Ntfy");
        menu_item.connect_activate( |item |{
            tray_menu_item_open_webbrowser(item.upcast_ref::<gtk::MenuItem>(),std::env::var("NTFY_URL").unwrap().as_str())
        });
        menu.append(&menu_item);
    };

    let menu_item = gtk::SeparatorMenuItem::default();
    menu.append(&menu_item);
    
    let menu_item = gtk::MenuItem::with_label("About");
    tray_menu_append_about_submenu2(
        &menu_item,
        config_file.clone(),
        got_url.clone(),
        got_token.clone(),
        nfy_url.clone(),
        nfy_topics.clone()
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
