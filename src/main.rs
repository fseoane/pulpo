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
    let url = base_url(&args.url)?;
    let ws_url = to_websocket(url.clone())?;
    let tokn = args.token.clone().unwrap();
    let poll = args.poll;

    println!("Starting with {} and token {}",url,tokn);
    println!("...and will poll every {} seconds",poll);

    println!("Creating gotify client");

    //daemonize the  process
    if !args.foreground{
        let daemonize = Daemonize::new();
        daemonize.start()?;
    }

    let gdnd_cli = GotifyWSClient::new(ws_url, tokn, None);
    match gdnd_cli.run_loop(poll) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }

}


fn main(){
    let cmdline: Vec<String> = std::env::args().collect();
    
    let mut config_filename: &str = "pulpo.conf";

    let path_and_prog_name = cmdline[0].as_str();
    let pos = path_and_prog_name.rfind('/').unwrap();
    let prog_name = &path_and_prog_name[(pos+1)..];
    let mut fg: bool = true;
    let mut c_url = Url::parse("https://yecla.mooo.com:20589/");
    let fg_option: &str = "-d";
    let config_option: &str = "-c";
    let help_option1: &str = "-h";
    let help_option2: &str = "--help";



    if cmdline.len()>1 && (cmdline[1].eq(help_option1) || cmdline[1].eq(help_option2)){
        println!(" ");
        println!("------------------------------------------------------------------------");
        println!("{} {} {}",prog_name,"[<options>]","[<url>]");
        println!("    Options:");
        println!("        -h     : This help.");
        println!("        --help");
        println!("        -d     : To run the program in background (daemonized).");
        println!("        <url>  : Gotify url in the form of http(s)://the.host.name:port");
        println!("                 Default <url> is https://yecla.mooo.com:20589");
        println!("------------------------------------------------------------------------");
        println!(" ");
        std::process::exit(1);
    }

    if cmdline.len()==2 {
        if cmdline[1].eq(fg_option){
            fg = false;
        };
        if cmdline[1].ne(fg_option) && cmdline[1].ne(help_option1) && cmdline[1].ne(help_option2){
            c_url = Url::parse(cmdline[1].as_str());    
        };
    };

    if cmdline.len()>2 && cmdline[1].eq(fg_option) && cmdline[1].ne(config_option){
        c_url = Url::parse(cmdline[2].as_str());
    };

    if cmdline.len()>2 && cmdline[1].ne(fg_option) && cmdline[1].eq(config_option){
        config_filename = cmdline[2].as_str();
    };
    
    let g_url = c_url;
    let args = Args { 
        token: Some("CCX6GtJPVh_osXB".to_string()), 
        url: g_url.unwrap(),
        poll: 5,
        foreground: fg,
    };

    let configdata: ConfigData = read_config(config_filename);

    let res: std::result::Result<(), PulpoError> = log_gotify_messages(args);
    println!("{}","Exiting");
    println!("{:#?}",res);

    // if let Err(e) = log_gotify_messages(args) {
    //     println!("{:#?}", e);
    //     println!("{}","Exiting");
    //     std::process::exit(1);
    // }

}