//use std::env::var;

//use structopt::StructOpt;
use url::Url;

// #[derive(Debug, Clone, StructOpt)]
// #[structopt(
//     name = "Gotify Desktop Notification Daemon",
//     about = "Receive notifications from gotify on your desktop."
// )]

#[derive(Debug)]
pub struct Args {
    /// Gotify client token: If used username password and client are not needed
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub gotify_token: String,

    /// Gotify server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_url: Url,

    /// Gotify notification sound
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_sound: String,
    
    /// Gotify notification icon
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_icon: String,

    /// Ntfy server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub ntfy_url: Url,

    /// Ntfy topics 
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_topics: String,
    
    /// Ntfy notification sound
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_sound: String,

    /// Ntfy notification icon
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_icon: String,

    /// Time between polling the gotify server in seconds
    //#[structopt(short = "P", long, default_value = "1", env = "GDND_POLL")]
    pub poll: u64,

    /// Run GDND in the foreground
    //#[structopt(short = "F", long)]
    pub foreground: bool,
}

#[derive(Debug)]
pub struct GotifyArgs {
    /// Gotify client token: If used username password and client are not needed
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub gotify_token: String,

    /// Gotify server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_url: Url,

    /// Gotify notification sound
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_sound: String,
    
    /// Gotify notification icon
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_icon: String,

    /// Time between polling the gotify server in seconds
    //#[structopt(short = "P", long, default_value = "1", env = "GDND_POLL")]
    pub poll: u64,

    /// Run GDND in the foreground
    //#[structopt(short = "F", long)]
    pub foreground: bool,
}

#[derive(Debug)]
pub struct NtfyArgs {
    /// Ntfy server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub ntfy_url: Url,

    /// Ntfy topics 
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_topics: String,
    
    /// Ntfy notification sound
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_sound: String,

    /// Ntfy notification icon
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_icon: String,

    /// Time between polling the gotify server in seconds
    //#[structopt(short = "P", long, default_value = "1", env = "GDND_POLL")]
    pub poll: u64,

    /// Run GDND in the foreground
    //#[structopt(short = "F", long)]
    pub foreground: bool,
}