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
    config::{ConfigData,read_config}, 
    tray::build_tray_menu, 
    helpers::{base_url, to_websocket},
    gotifywsclient::GotifyWSClient,
    ntfywsclient::NtfyWSClient,
};

use log::info;
use url::Url;
use daemonize::Daemonize;

type Result<T> = std::result::Result<T, PulpoError>;

// --------------------------------------------------------------------------------------------------

fn log_gotify_messages(args: GotifyArgs) -> Result<()> {
    // make sure the URL is clean
    let url = helpers::base_url(&args.gotify_url)?;
    let ws_url = helpers::to_websocket(url.clone())?;
    let tokn = args.gotify_token.clone().unwrap();
    let poll = args.poll;
    let sound = args.gotify_sound.clone().unwrap();
    let icon = args.gotify_icon.clone().unwrap();

    info!("Starting gotify with {} and token {}",url,tokn);
    info!("...and will poll every {} seconds",poll);


    //daemonize the  process
    if !args.foreground{
        info!("Starting daemon.");
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
    let topics = args.ntfy_topics.clone().unwrap();
    let ws_url = helpers::to_websocket(url.clone())?;

    let poll = args.poll;
    let sound = args.ntfy_sound.clone().unwrap();
    let icon = args.ntfy_icon.clone().unwrap();

    info!("Starting ntfy with {} and topics {}",url,topics);
    info!("...and will poll every {} seconds",poll);

    //daemonize the  process
    if !args.foreground{
        info!("Starting daemon.");
        let daemonize = Daemonize::new();
        let _ = daemonize.execute();
        //daemonize.start()?;
    }

    //Creating the client and looping
    info!("Creating ntify client");
    let ntfy_cli = NtfyWSClient::new(ws_url, topics);
    match ntfy_cli.run_loop(poll,sound.as_str(),icon.as_str()) {
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
    let mut fg: bool = true;
    let fg_option: &str = "-d";
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

    let mut has_gotify_config: bool = false;
    let mut has_ntfy_config: bool = false;
    let mut gotify_args: GotifyArgs;
    let mut ntfy_args: NtfyArgs;

    let mut got_url: Url;
    let mut got_token: &str;
    let mut got_sound: &str;
    let mut got_icon: &str;

    let mut nfy_url: Url;
    let mut nfy_topics: &str;
    let mut nfy_sound: &str;;
    let mut nfy_icon: &str;

    println!("Reading config from:            {}", config_filename); 
    println!("------------------------------------------------------------------------");
    let configdata: ConfigData = read_config(config_filename);
    
    if configdata.config.tray_icon.len()>0 {
        // Print out the values to `stdout`.
        println!("    config/tray_icon:           {}", configdata.config.tray_icon); 
    }


    //if Some(pulpo::config::ConfigData configdata.config) 
    if configdata.gotify.unwrap().gotify_url.len()>0 {
        has_gotify_config = true;
        got_url = Url::parse(configdata.gotify.unwrap().gotify_url.as_str());
        got_token = configdata.gotify.unwrap().gotify_client_token.as_str();
        got_sound = configdata.gotify.unwrap().gotify_sound.as_str();
        got_icon = configdata.gotify.unwrap().gotify_icon.as_str();
        gotify_args = GotifyArgs { 
            gotify_token: Some(got_token.to_string()), 
            gotify_url: got_url,
            gotify_sound: Some(got_sound.to_string()),
            gotify_icon: Some(got_icon.to_string()),
            poll: 5,
            foreground: fg,
        };
        println!("    gotify/gotify_url:          {}", got_url.as_str());
        println!("    gotify/gotify_client_token: {}", got_token);
        println!("    gotify/gotify_sound:        {}", got_sound);
        println!("    gotify/gotify_icon:         {}", got_icon);

    };

    if configdata.ntfy.unwrap().ntfy_url.len()>0  {
        has_ntfy_config = true;
        nfy_url = Url::parse(configdata.ntfy.unwrap().ntfy_url.as_str());
        nfy_topics = configdata.ntfy.unwrap().ntfy_topics.as_str();
        nfy_sound = configdata.ntfy.unwrap().ntfy_sound.as_str();
        nfy_icon = configdata.ntfy.unwrap().ntfy_icon.as_str();
        
        ntfy_args = NtfyArgs { 
            ntfy_url: nfy_url,
            ntfy_topics: Some(nfy_topics.to_string()), 
            ntfy_sound: Some(nfy_sound.to_string()), 
            ntfy_icon: Some(nfy_icon.to_string()), 
            poll: 5,
            foreground: fg,
        };
        println!("    ntfy/ntfy_url:              {}", nfy_url.as_str());
        println!("    ntfy/ntfy_topics:           {}", nfy_topics);
        println!("    ntfy/ntfy_sound:            {}", nfy_sound);
        println!("    ntfy/ntfy_icon:             {}", nfy_icon);
    };
    println!("------------------------------------------------------------------------");
    println!(" ");

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
        build_tray_menu(config_filename,configdata);
    };

    if has_gotify_config && !has_ntfy_config{
        let gotify_thread = || {
            let gtfy_res: std::result::Result<(), PulpoError> = log_gotify_messages(gotify_args);
            info!("{}","Exiting");
            info!("Gotify result: {:#?}",gtfy_res);
        };
        std::thread::scope(|s| {
            s.spawn(tray_thread);
            s.spawn(gotify_thread);
            
        });
    };

    if !has_gotify_config && has_ntfy_config{
        let ntfy_thread = || {
            let ntfy_res: std::result::Result<(), PulpoError> = log_ntfy_messages(ntfy_args);
            info!("{}","Exiting");
            info!("Ntfy result: {:#?}",ntfy_res);
        };
        std::thread::scope(|s| {
            s.spawn(tray_thread);
            s.spawn(ntfy_thread);
        });
    };

    if has_gotify_config && has_ntfy_config{
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
            s.spawn(gotify_thread);
            s.spawn(ntfy_thread);
        });
    }
}