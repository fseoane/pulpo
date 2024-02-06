// Import the required dependencies.
use serde_derive::{Serialize, Deserialize};
use std::fs;
use toml;
use log::{error, info, warn};



// Top level struct to hold the TOML data.
#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub config: GeneralConfig,
	pub gotify: Option<GotifyConf>,
	pub ntfy: Option<NtfyConf>,
}

// Config struct holds to data from the `[config]` section.
#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub tray_icon: String,
}

// Config struct holds to data from the `[gotify]` section.
#[derive(Serialize, Deserialize)]
pub struct GotifyConf {
	pub gotify_url: String,
	pub gotify_client_token: String,
    pub gotify_sound: String,   
    pub gotify_icon: String,
}

// Config struct holds to data from the `[ntfy]` section.
#[derive(Serialize, Deserialize)]
pub struct NtfyConf {
	pub ntfy_url: String,
	pub ntfy_topics: String,
    pub ntfy_sound: String,
    pub ntfy_icon: String,
}

pub fn read_config(filename: &str) -> ConfigData{
    // Read the contents of the file using a `match` block 
    // to return the `data: Ok(c)` as a `String` 
    // or handle any `errors: Err(_)`.
    info!("Configuration ({}) : ", filename);
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