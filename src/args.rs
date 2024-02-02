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
    pub gotify_token: Option<String>,

    /// Gotify username
    //#[structopt(short = "U", long, env = "GDND_USER")]
    //pub username: Option<String>,

    /// Gotify password
    //#[structopt(short, long, env = "GDND_PASSWORD")]
    //pub password: Option<String>,

    /// Gotify server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub gotify_url: Url,

    /// Ntfy server url
    //#[structopt(short, long, env = "GDND_URL", parse(try_from_str = Url::parse))]
    pub ntfy_url: Url,

    /// Gotify client token: If used username password and client are not needed
    //#[structopt(short = "t", long, env = "GDND_TOKEN")]
    pub ntfy_topics: Option<String>,

    /// Gotify client name: Required if authenticating with username and password
    //#[structopt(short, long, env = "GDND_CLIENT")]
    //pub client: Option<String>,

    /// Time between polling the gotify server in seconds
    //#[structopt(short = "P", long, default_value = "1", env = "GDND_POLL")]
    pub poll: u64,

    /// Run GDND in the foreground
    //#[structopt(short = "F", long)]
    pub foreground: bool,
}
