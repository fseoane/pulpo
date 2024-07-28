
use std::env;
use std::thread;
use std::time::Duration;
extern crate ears;
use ears::{Sound, AudioController};

use crate::errors::PulpoError;
//use crate::helpers::{get_cache_path, to_websocket};

use log::{info, warn};
use notify_rust::{Notification,Timeout};
use serde::{Deserialize, Serialize};
use tungstenite::Message;
use url::Url;
use chrono::{DateTime, Utc};
use serde_json::Value;

type Result<T> = std::result::Result<T, PulpoError>;

// ntfy api structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    pub messages: Vec<NtfyMessage>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NtfyMessage {
    pub id: String,
    pub time: usize,
    pub event: String,
    pub topic: String,
    pub expires: Option<usize>,
    pub message: Option<String>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<u8>,
    pub click: Option<Url>,
    pub actions: Option<Vec<Value>>,
    pub attachment: Option<Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NtfyMessageAttachment {
    pub name: String,
    pub url: Url,
    pub typea: Option<String>,
    pub size: Option<usize>,
    pub expires: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paging {
    pub limit: usize,
    pub next: Option<String>,
    pub since: usize,
    pub size: usize,
}

#[derive(Deserialize, Serialize)]
pub struct NtfyWSClient {
    ws_url: Url,
    topics: String,
}

impl NtfyWSClient {
    pub fn new(ws_url: Url, topics: String) -> Self {
        NtfyWSClient {
            ws_url,
            topics,
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

    pub fn run_loop(&self, poll: u64, notif_sound: &str, notif_icon: &str, notification_timeout_secs: u32) -> Result<()> {
        
        info!("{}","Starting ntfy loop");

        // TO DO: factor out the connection
        let mut ws_url = self.ws_url.clone();
        let url_path = format!("{}/{}",self.topics,"ws");
        ws_url.set_path(url_path.as_str());


        info!("Ntfy websocket url: {}", ws_url);

        let (mut socket, 
            _response) = tungstenite::connect(ws_url.as_str())?;

        if socket.can_read(){
            info!("Connected to Ntfy at {}", self.ws_url);
        }

        let mut notif: std::result::Result<notify_rust::NotificationHandle, notify_rust::error::Error>;

        loop {
            // attempt to read from the socket
            let message: Option<NtfyMessage> = match socket.read()? {
                Message::Text(s) => {
                    Some(serde_json::from_str(&s)?)
                }
                _ => None,
            };
            
            // if a message was received create a notification
            if let Some(m) = message {
                
                if m.event=="message"{
                    let tit = m.title.clone().unwrap();
                    let messge = m.message.clone().unwrap();

                    info!("[✉]Ntfy message received | title:{} message:{}",
                        format!("{}/{}",&m.topic,&tit).as_str(),
                        &messge
                    );

                    info!("    DND:           {}", std::env::var("DND").unwrap()); 
                    info!("    SILENT:        {}", std::env::var("SILENT").unwrap());  

                    if std::env::var("SILENT").unwrap()=="off" && std::env::var("DND").unwrap()=="off"{
                        NtfyWSClient::play_file(format!("/opt/pulpo/resources/{}",notif_sound).as_str());
                    };

                    if std::env::var("DND").unwrap()=="off"{
                        notif = Notification::new()
                            .summary(format!("{}/{}",&m.topic,&tit).as_str())
                            .body(&messge)
                            .appname("pulpo")
                            .icon(format!("/opt/pulpo/resources/{}",notif_icon).as_str())
                            .image_path(format!("/opt/pulpo/resources/{}",notif_icon).as_str())
                            .timeout(Timeout::Milliseconds(notification_timeout_secs*1000))
                            .show();
                    
                        // if the notification fails some how log it but do not kill the process
                        // TO DO: Add tracking for the number of failaures and perhaps have it exit after a certain configurable
                        // threshhold
                        match notif {
                            Ok(_) => info!(
                                "[✉] Sent Ntfy desktop notification: title: {} message: {}",
                                format!("{}:{}",&m.topic,&tit).as_str(), 
                                &messge
                            ),
                            Err(e) => warn!("[!]Failed to send Ntfy desktop notification: {}", e),
                        };
                    };
                }
            }
            thread::sleep(Duration::from_secs(poll));
        }
    }
}
