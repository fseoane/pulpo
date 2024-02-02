pub mod args;
pub mod errors;
pub mod helpers;
pub mod tray;
pub mod config;
pub mod gotifywsclient;

use crate::{
    args::Args,
    errors::PulpoError,
    config::{ConfigData,read_config}, 
    tray::build_tray_menu, 
    gotifywsclient::GotifyWSClient,
    helpers::{base_url, to_websocket},
};

use std::path::Path;
use std::time::Duration;
use std::env::var;
use chrono::{DateTime, Utc};


use log::{info, warn};
use url::{ParseError, Url};
use daemonize::Daemonize;

type Result<T> = std::result::Result<T, PulpoError>;

// --------------------------------------------------------------------------------------------------

fn log_gotify_messages(args: Args) -> Result<()> {
    // make sure the URL is clean
    let url = base_url(&args.gotify_url)?;
    let ws_url = to_websocket(url.clone())?;
    let tokn = args.gotify_token.clone().unwrap();
    let poll = args.poll;

    println!("Starting with {} and token {}",url,tokn);
    println!("...and will poll every {} seconds",poll);


    //daemonize the  process
    if !args.foreground{
        println!("Starting daemon.");
        let daemonize = Daemonize::new();
        daemonize.start()?;
    }

    println!("Creating gotify client");
    let gdnd_cli = GotifyWSClient::new(ws_url, tokn, None);
    match gdnd_cli.run_loop(poll) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }

}


fn main(){
    let cmdline: Vec<String> = std::env::args().collect();
    
    let mut config_filename: &str = "/etc/pulpo.conf";

    let path_and_prog_name = cmdline[0].as_str();
    let filename_start = path_and_prog_name.rfind('/').unwrap();
    let prog_name = &path_and_prog_name[(filename_start+1)..];
    let mut fg: bool = false;
    let fg_option: &str = "-f";
    let config_option: &str = "-c";
    let help_option1: &str = "-h";
    let help_option2: &str = "--help";



    if cmdline.iter().any(|i| i==help_option1) || cmdline.iter().any(|i| i==help_option2) {
        println!(" ");
        println!("------------------------------------------------------------------------");
        println!("{} {} {}",prog_name,"[<options>]","[-c <file>]");
        println!("    Options:");
        println!("        -h        : This help.");
        println!("        --help");
        println!("        -f        : To run the program in foreground (not daemonized).");
        println!("    Optional config file:");
        println!("        -c <file> : Specify the config file");
        println!("                    NOTE: By default config file is /etc/pulpo.conf");
        println!("------------------------------------------------------------------------");
        println!(" ");
        std::process::exit(1);
    }


    // Only one parameter (after the parameter 0 corresponding to the program name itself)
    if cmdline.iter().any(|i| i==fg_option) {  // tag = -f (run in foreground - not daemonized)
        fg = true;
    };
    
    if cmdline.iter().any(|i| i==config_option) {  // tag = -f (run in foreground - not daemonized)
        let pos: std::option::Option<usize> = Some(cmdline.iter().rposition(|i| i==config_option).unwrap());
        if pos.is_some() {
            config_filename = cmdline[pos.unwrap()+1].as_str();
        }
    };


    println!("Reading config from:            {}", config_filename); 
    println!("------------------------------------------------------------------------");
    let configdata: ConfigData = read_config(config_filename);
    // Print out the values to `stdout`.
    println!("    config/tray_icon:           {}", configdata.config.tray_icon); 
    println!("    gotify/gotify_url:          {}", configdata.gotify.gotify_url);
	println!("    gotify/gotify_client_token: {}", configdata.gotify.gotify_client_token);
	println!("    gotify/gotify_sound:        {}", configdata.gotify.gotify_sound);
	println!("    ntfy/ntfy_url:              {}", configdata.ntfy.ntfy_url);
	println!("    ntfy/ntfy_topics:           {}", configdata.ntfy.ntfy_topics);
	println!("    ntfy/ntfy_sound:            {}", configdata.ntfy.ntfy_sound);
    println!("------------------------------------------------------------------------");
    println!(" ");
        
    let got_url = Url::parse(configdata.gotify.gotify_url.as_str());
    let got_token = configdata.gotify.gotify_client_token.as_str();
    let nfy_url = Url::parse(configdata.ntfy.ntfy_url.as_str());
    let nfy_topics = configdata.ntfy.ntfy_topics.as_str();
    
    let args = Args { 
        gotify_token: Some(got_token.to_string()), 
        gotify_url: got_url.unwrap(),
        ntfy_url: nfy_url.unwrap(),
        ntfy_topics: Some(nfy_topics.to_string()), 
        poll: 5,
        foreground: fg,
    };

    //let icon_filename = configdata.config.tray_icon.as_str();
    let tray_thread = || {
        let icon_filename = configdata.config.tray_icon.as_str();
        build_tray_menu(icon_filename);
    };

    let gotify_thread = || {
        let res: std::result::Result<(), PulpoError> = log_gotify_messages(&args);
        println!("{}","Exiting");
        println!("{:#?}",res);
    };

    std::thread::scope(|s| {
        s.spawn(tray_thread);
        s.spawn(gotify_thread);
    });

    // let tray_thread = std::thread::spawn(move || {
    //     let icon_filename = configdata.config.tray_icon.as_str();
    //     build_tray_menu(icon_filename);
    // });

    // let gotify_thread = std::thread::spawn(move || {
    //     let res: std::result::Result<(), PulpoError> = log_gotify_messages(args);
    //     println!("{}","Exiting");
    //     println!("{:#?}",res);
    // });

    // println!("{}","Exiting");
    // println!("{:#?}",res);

    // // if let Err(e) = log_gotify_messages(args) {
    // //     println!("{:#?}", e);
    // //     println!("{}","Exiting");
    // //     std::process::exit(1);
    // // }

}