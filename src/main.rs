// Ref: https://gitlab.com/cosmo_duff/gotify-desktop-notification-daemon/-/tree/main/src?ref_type=heads


pub mod args;
pub mod errors;
pub mod helpers;
pub mod tray;
pub mod config;
pub mod gotifywsclient;
pub mod ntfywsclient;


use crate::{
    args::{Args,GotifyArgs,NtfyArgs},
    errors::PulpoError,
    config::{read_config,NtfyConf, ConfigData, GotifyConf}, 
    tray::build_tray_menu, 
    helpers::{base_url, to_websocket},
    gotifywsclient::GotifyWSClient,
    ntfywsclient::NtfyWSClient,
};

use std::env;
use url::Url;
use daemonize::Daemonize;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use simplelog::{Config, LevelFilter, SimpleLogger,WriteLogger};
use std::fs::File;
use single_instance::SingleInstance;

type Result<T> = std::result::Result<T, PulpoError>;

// --------------------------------------------------------------------------------------------------
fn active_internet_connection() -> bool {
    // let mut stream = TcpStream::connect("8.8.8.8");

    // stream.write(&[1]);
    // stream.read(&mut [0; 128]);
    // Ok((true))

    match TcpStream::connect("8.8.8.8:53"){
        std::io::Result::Ok(_s) => true,
        std::io::Result::Err(_e) => false,
    }
}

fn log_gotify_messages(args: GotifyArgs) -> Result<()> {
    // make sure the URL is clean
    let url = helpers::base_url(&args.gotify_url)?;
    let ws_url = helpers::to_websocket(url.clone())?;
    let tokn = args.gotify_token.clone();
    let poll = args.poll;
    let sound = args.gotify_sound.clone();
    let icon = args.gotify_icon.clone();

    info!("Starting gotify with {} and token {}",url,tokn);
    info!("...and will poll every {} seconds",poll);


    //daemonize the  process
    if !args.foreground{
        info!("Starting ntfy daemon.");
        let daemonize = Daemonize::new();
        let _ = daemonize.execute();
        //daemonize.start()?;
    }

    //Creating the client and looping
    info!("Creating gotify client");
    let gotify_cli = GotifyWSClient::new(ws_url, tokn, None);
    match gotify_cli.run_loop(poll,sound.as_str(),icon.as_str()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }

}

fn log_ntfy_messages(args: NtfyArgs) -> Result<()> {
    // make sure the URL is clean

    let url = helpers::base_url(&args.ntfy_url)?;
    let topics = args.ntfy_topics.clone();
    let ws_url = helpers::to_websocket(url.clone())?;

    let poll = args.poll;
    let sound = args.ntfy_sound.clone();
    let icon = args.ntfy_icon.clone();

    info!("Starting ntfy with {} and topics {}",url,topics);
    info!("...and will poll every {} seconds",poll);

    //daemonize the  process
    if !args.foreground{
        info!("Starting ntfy daemon.");
        let daemonize = Daemonize::new();
        let _ = daemonize.execute();
        //daemonize.start()?;
    }

    //Creating the client and looping
    info!("Creating ntfy client");
    let ntfy_cli = NtfyWSClient::new(ws_url, topics);
    match ntfy_cli.run_loop(poll,sound.as_str(),icon.as_str()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }

}



fn main(){
    
    let instance = SingleInstance::new("whatever").unwrap();
    if !instance.is_single(){
        eprintln!("Failed to initiate program (another instance is already running)");
        std::process::exit(1);
    };
    
    
    let cmdline: Vec<String> = std::env::args().collect();
    
    let mut config_filename: &str = "/etc/pulpo.conf";

    let path_and_prog_name = cmdline[0].as_str();
    let filename_start = path_and_prog_name.rfind('/').unwrap();
    let prog_name = &path_and_prog_name[(filename_start+1)..];
    let mut fg: bool = true;
    let fg_option: &str = "-d";
    let config_option: &str = "-c";
    let help_option1: &str = "-h";
    let help_option2: &str = "--help";

    env::set_var("SILENT", String::from("off"));
    env::set_var("DND", String::from("off"));

    // if let Err(e) = SimpleLogger::init(LevelFilter::Info,Config::default()) {
    //     eprintln!("Failed to initiate the logger: {}", e);
    //     std::process::exit(1);
    // } else {
    //     let _ = WriteLogger::new(
    //         LevelFilter::Info,
    //         Config::default(),
    //         File::create("pulpo.log").unwrap(),
    //     );
    // };

    if let Err(e) = WriteLogger::init(
            LevelFilter::Info,
            Config::default(),
            File::create("pulpo.log").unwrap(),
        ){
        eprintln!("Failed to initiate the logger: {}", e);
        std::process::exit(1);
    };

    if cmdline.iter().any(|i| i==help_option1) || cmdline.iter().any(|i| i==help_option2) {
        println!(" ");
        println!("------------------------------------------------------------------------");
        println!("{} {} {}",prog_name,"[<options>]","[-c <file>]");
        println!("    Options:");
        println!("        -h        : This help.");
        println!("        --help");
        println!("        -d        : To run the program in background (daemonized).");
        println!("    Optional config file:");
        println!("        -c <file> : Specify the config file");
        println!("                    NOTE: By default config file is /etc/pulpo.conf");
        println!("------------------------------------------------------------------------");
        println!(" ");
        std::process::exit(1);
    }

    // Only one parameter (after the parameter 0 corresponding to the program name itself)
    if cmdline.iter().any(|i| i==fg_option) {  // tag = -f (run in foreground - not daemonized)
        fg = false;
    };
    
    if cmdline.iter().any(|i| i==config_option) {  // tag = -f (run in foreground - not daemonized)
        let pos: std::option::Option<usize> = Some(cmdline.iter().rposition(|i| i==config_option).unwrap());
        if pos.is_some() {
            config_filename = cmdline[pos.unwrap()+1].as_str();
        }
    };

    let mut gotify_args= GotifyArgs { 
        gotify_token: String::from(""), 
        gotify_url: url::Url::parse("https://1.1.1.1").unwrap(),
        gotify_sound: String::from(""),
        gotify_icon: String::from(""),
        poll: 5,
        foreground: true,
    };

    let mut ntfy_args= NtfyArgs { 
        ntfy_topics: String::from(""), 
        ntfy_url: url::Url::parse("https://1.1.1.1").unwrap(),
        ntfy_sound: String::from(""),
        ntfy_icon: String::from(""),
        poll: 5,
        foreground: true,
    };

    let got_url;
    let got_token: &str;
    let got_sound: &str;
    let got_icon: &str;

    let nfy_url;
    let nfy_topics: &str;
    let nfy_sound: &str;
    let nfy_icon: &str;

    info!("Reading config from:            {}", config_filename); 
    info!("------------------------------------------------------------------------");
    println!("Reading config from:            {}", config_filename); 
    println!("------------------------------------------------------------------------");

    let configdata: ConfigData = read_config(config_filename);
    
    let has_gotify_config= configdata.gotify.is_some();
    let mut gotify_conf = GotifyConf { 
        gotify_client_token: String::from(""), 
        gotify_url: String::from(""),
        gotify_sound: String::from(""),
        gotify_icon: String::from(""),
    };

    if has_gotify_config{
        gotify_conf = configdata.gotify.unwrap();
    };
    
    let has_ntfy_config= configdata.ntfy.is_some();
    let mut ntfy_conf= NtfyConf { 
        ntfy_topics: String::from(""), 
        ntfy_url: String::from(""),
        ntfy_sound: String::from(""),
        ntfy_icon: String::from(""),
    };

    if has_ntfy_config {
        ntfy_conf= configdata.ntfy.unwrap();
    };

    let mut tray_icon: &str = "";
    if configdata.config.tray_icon.len()>0 {
        // Print out the values to `stdout`.
        tray_icon = configdata.config.tray_icon.as_str();
        info!("    config/tray_icon:           {}", tray_icon); 
        println!("    config/tray_icon:           {}", tray_icon); 
    }


    if has_gotify_config {
    //if configdata.gotify.as_ref().unwrap().gotify_url.len()>0 {
        got_url = Url::parse(gotify_conf.gotify_url.as_str());
        got_token = gotify_conf.gotify_client_token.as_str();
        got_sound = gotify_conf.gotify_sound.as_str();
        got_icon = gotify_conf.gotify_icon.as_str();
        gotify_args = GotifyArgs { 
            gotify_token: String::from(got_token), 
            gotify_url: got_url.unwrap(),
            gotify_sound: String::from(got_sound),
            gotify_icon: String::from(got_icon),
            poll: 5,
            foreground: fg,
        };
        info!("    gotify/gotify_url:          {}", gotify_args.gotify_url.as_str());
        info!("    gotify/gotify_client_token: {}", gotify_args.gotify_token.as_str());
        info!("    gotify/gotify_sound:        {}", gotify_args.gotify_sound.as_str());
        info!("    gotify/gotify_icon:         {}", gotify_args.gotify_icon.as_str());
        println!("    gotify/gotify_url:          {}", gotify_args.gotify_url.as_str());
        println!("    gotify/gotify_client_token: {}", gotify_args.gotify_token.as_str());
        println!("    gotify/gotify_sound:        {}", gotify_args.gotify_sound.as_str());
        println!("    gotify/gotify_icon:         {}", gotify_args.gotify_icon.as_str());

    };

    //let ntfy_cfg = configdata.ntfy.as_ref().unwrap().clone();
    if has_ntfy_config  {
    //if configdata.ntfy.as_ref().unwrap().ntfy_url.len()>0  {
        nfy_url = Url::parse(ntfy_conf.ntfy_url.as_str());
        nfy_topics = ntfy_conf.ntfy_topics.as_str();
        nfy_sound = ntfy_conf.ntfy_sound.as_str();
        nfy_icon = ntfy_conf.ntfy_icon.as_str();
        
        ntfy_args = NtfyArgs { 
            ntfy_url: nfy_url.unwrap(),
            ntfy_topics: String::from(nfy_topics), 
            ntfy_sound: String::from(nfy_sound), 
            ntfy_icon: String::from(nfy_icon), 
            poll: 5,
            foreground: fg,
        };
        info!("    ntfy/ntfy_url:              {}", ntfy_args.ntfy_url.as_str());
        info!("    ntfy/ntfy_topics:           {}", ntfy_args.ntfy_topics.as_str());
        info!("    ntfy/ntfy_sound:            {}", ntfy_args.ntfy_sound.as_str());
        info!("    ntfy/ntfy_icon:             {}", ntfy_args.ntfy_icon.as_str());
        println!("    ntfy/ntfy_url:              {}", ntfy_args.ntfy_url.as_str());
        println!("    ntfy/ntfy_topics:           {}", ntfy_args.ntfy_topics.as_str());
        println!("    ntfy/ntfy_sound:            {}", ntfy_args.ntfy_sound.as_str());
        println!("    ntfy/ntfy_icon:             {}", ntfy_args.ntfy_icon.as_str());

    };
    info!("------------------------------------------------------------------------");
    info!(" ");
    println!("------------------------------------------------------------------------");
    println!(" ");

    // Wait for internet connection to be available
    let mut counter = 0;

    info!("Waiting for network connection.");
    while counter < 12 && !active_internet_connection() { 
        thread::sleep(Duration::from_secs(5));
        counter += 1;
    };
    info!("Network is avaible.");
    println!("Network is avaible.");
 

    // let got_url = Url::parse(configdata.gotify.unwrap().gotify_url.as_str());
    // let got_token = configdata.gotify.unwrap().gotify_client_token.as_str();
    // let got_sound = configdata.gotify.unwrap().gotify_sound.as_str();
    // let got_icon = configdata.gotify.unwrap().gotify_icon.as_str();
    // let nfy_url = Url::parse(configdata.ntfy.ntfy_url.as_str());
    // let nfy_topics = configdata.ntfy.ntfy_topics.as_str();
    // let nfy_sound = configdata.ntfy.ntfy_sound.as_str();
    // let nfy_icon = configdata.ntfy.ntfy_icon.as_str();

    let tray_thread = || {
        //build_tray_menu(icon_filename);
        build_tray_menu(
            config_filename,
            tray_icon,
            gotify_conf.gotify_url.as_str(),
            gotify_conf.gotify_client_token.as_str(),
            ntfy_conf.ntfy_url.as_str(),
            ntfy_conf.ntfy_topics.as_str(),
        );
    };

    let gotify_thread = || {
        let gtfy_res: std::result::Result<(), PulpoError> = log_gotify_messages(gotify_args);
        info!("{}","Exiting");
        info!("Gotify result: {:#?}",gtfy_res);
    };


    let ntfy_thread = || {
        let ntfy_res: std::result::Result<(), PulpoError> = log_ntfy_messages(ntfy_args);
        info!("{}","Exiting");
        info!("Ntfy result: {:#?}",ntfy_res);
    };
    
    std::thread::scope(|s| {
        s.spawn(tray_thread);
        if has_gotify_config {
            s.spawn(gotify_thread);
        }
        if has_ntfy_config{
            s.spawn(ntfy_thread);
        }
    });
    
}