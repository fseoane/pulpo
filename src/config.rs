

// Import the required dependencies.
use serde_derive::Deserialize;
use std::fs;
use toml;
//use std::path::Path;



// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct ConfigData {
    pub config: GeneralConfig,
	pub gotify: GotifyConf,
	pub ntfy: NtfyConf,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct GeneralConfig {
    pub tray_icon: String,
}

// Config struct holds to data from the `[gotify]` section.
#[derive(Deserialize)]
pub struct GotifyConf {
	pub gotify_url: String,
	pub gotify_client_token: String,
	pub gotify_sound: String,
}

// Config struct holds to data from the `[ntfy]` section.
#[derive(Deserialize)]
pub struct NtfyConf {
	pub ntfy_url: String,
	pub ntfy_topics: String,
	pub ntfy_sound: String,
}

pub fn read_config(filename: &str) -> ConfigData{
    // Read the contents of the file using a `match` block 
    // to return the `data: Ok(c)` as a `String` 
    // or handle any `errors: Err(_)`.
    eprintln!("Configuration ({}) : ", filename);
    let contents:String = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("[!] Could not read config file `{}`", filename);
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };

    // Use a `match` block to return the 
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let configdata: ConfigData = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("[!] Unable to load config data from `{}`", filename);
            // Exit the program with exit code `1`.
            std::process::exit(1);
        }
    };
    return configdata;
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


// }