
use std::env;
use std::thread;
use std::time::Duration;
extern crate ears;
use ears::{Sound, AudioController};

use crate::errors::PulpoError;
//use crate::helpers::{get_cache_path, to_websocket};

use log::{info, warn};
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

    fn play_file(file: &str) {
        // Create a new Sound.
        let mut snd = Sound::new(file).unwrap();

        // Play the Sound
        snd.play();

        // Wait until the end of the sound
        while snd.is_playing() {}
    }

    pub fn run_loop(&self, poll: u64, notif_sound: &str, notif_icon: &str) -> Result<()> {
        
        info!("{}","Starting gotify loop");

        // TO DO: factor out the connection
        let mut ws_url = self.ws_url.clone();
        ws_url.set_path("stream");
        let query = format!("token={}", self.token);
        ws_url.set_query(Some(&query));

        info!("Gotify websocket url: {}", ws_url);

        let (mut socket, _response) = tungstenite::connect(&ws_url)?;

        if socket.can_read(){
            info!("Connected to Gotify at {}", self.ws_url);
        }
        
        loop {
            // attempt to read from the socket
            // let message: Option<GotifyMessage> = match socket.read_message()? {
            let message: Option<GotifyMessage> = match socket.read()? {
                Message::Text(s) => {
                    Some(serde_json::from_str(&s)?)
                }
                _ => None,
            };

            // if a message was received create a notification
            if let Some(m) = message {
                info!("[✉] Gotify message received | title:{} message:{}",m.title,m.message);

                if std::env::var("SILENT").unwrap()=="off" && std::env::var("DND").unwrap().as_str()=="off"{
                    GotifyWSClient::play_file(format!("resources/{}",notif_sound).as_str());
                };
                if std::env::var("DND").unwrap()=="off"{
                    let notif = Notification::new()
                        .summary(&m.title)
                        .body(&m.message)
                        .icon(format!("/opt/pulpo/resources/{}",notif_icon).as_str())
                        .show();

                    // if the notification fails some how log it but do not kill the process
                    // TO DO: Add tracking for the number of failaures and perhaps have it exit after a certain configurable
                    // threshhold
                    match notif {
                        Ok(_) => info!(
                            "[✉] Sent gotify desktop notification: title: {} message: {}",
                            m.title, m.message
                        ),
                        Err(e) => warn!("[!] Failed to send gotify desktop notification: {}", e),
                    }
                };  

            }
            thread::sleep(Duration::from_secs(poll));
        }

    }
}
