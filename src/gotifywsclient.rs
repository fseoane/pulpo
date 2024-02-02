use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::errors::PulpoError;
use crate::helpers::{get_cache_path, to_websocket};

use log::{debug, info, warn};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use tungstenite::Message;
use url::Url;
use chrono::{DateTime, Utc};
use serde_json::Value;

type Result<T> = std::result::Result<T, PulpoError>;

type AuthToken = String;

// gotify api structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    pub messages: Vec<GotifyMessage>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GotifyMessage {
    pub appid: usize,
    pub date: DateTime<Utc>,
    pub extras: Option<Vec<Value>>,
    pub id: usize,
    pub message: String,
    pub priority: usize,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paging {
    pub limit: usize,
    pub next: Option<String>,
    pub since: usize,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: usize,
    pub name: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct GotifyWSClient {
    ws_url: Url,
    token: AuthToken,
    client_name: Option<String>,
}

impl GotifyWSClient {
    pub fn new(ws_url: Url, token_provider: AuthToken, client_name: Option<String>) -> Self {
        GotifyWSClient {
            ws_url,
            token: token_provider,
            client_name,
        }
    }

    // pub fn from_cache(client_name: &str) -> Result<Self> {
    //     let cache_path = get_cache_path()?;
    //     let json_name = format!("{}/{}.json", &cache_path, client_name);
    //     let json_file = Path::new(&json_name);

    //     let f = fs::File::open(json_file)?;
    //     let reader = BufReader::new(f);

    //     let c = serde_json::from_reader(reader)?;

    //     return Ok(c);
    // }

    // pub fn from_user_auth(auth: UserAuth) -> Result<Self> {
    //     let cli = auth.authenticate()?;
    //     let ws_url = to_websocket(auth.url)?;

    //     let gdnd_cli = GotifyWSClient::new(ws_url, cli.token, Some(auth.client));

    //     Ok(gdnd_cli)
    // }

    // pub fn write_cache(&self) -> Result<()> {
    //     if let Some(c) = &self.client_name {
    //         let cache_path = get_cache_path()?;
    //         fs::create_dir_all(&cache_path)?;
    //         let json_file = format!("{}/{}.json", &cache_path, c);
    //         let f = fs::File::create(json_file)?;

    //         serde_json::to_writer(f, &self)?;

    //         Ok(())
    //     } else {
    //         let err_msg = "Missing client name unable to write cache.".to_string();
    //         Err(PulpoError::MissingArgs(err_msg))
    //     }
    // }

    pub fn run_loop(&self, poll: u64) -> Result<()> {
        // TO DO: factor out the connection
        let mut ws_url = self.ws_url.clone();
        ws_url.set_path("stream");
        let query = format!("token={}", self.token);
        ws_url.set_query(Some(&query));
        debug!("Websocket url: {}", ws_url);
        let (mut socket, _response) = tungstenite::connect(&ws_url)?;
        info!("Connected to {}", self.ws_url);

        loop {
            // attempt to read from the socket
            let message: Option<GotifyMessage> = match socket.read_message()? {
                Message::Text(s) => {
                    info!("Message received");
                    Some(serde_json::from_str(&s)?)
                }
                _ => None,
            };

            // if a message was received create a notification
            if let Some(m) = message {
                let notif = Notification::new()
                    .summary(&m.title)
                    .body(&m.message)
                    .show();

                // if the notification fails some how log it but do not kill the process
                // TO DO: Add tracking for the number of failaures and perhaps have it exit after a certain configurable
                // threshhold
                match notif {
                    Ok(_) => info!(
                        "Sent desktop notification: title: {} message: {}",
                        m.title, m.message
                    ),
                    Err(e) => warn!("Failed to send desktop notification: {}", e),
                }
            }

            thread::sleep(Duration::from_secs(poll));
        }
    }
}
