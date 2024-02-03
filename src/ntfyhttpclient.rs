use std::thread;
use std::time::Duration;
extern crate ears;
use ears::{Sound, AudioController};
use log::{info, warn};
use crate::errors::PulpoError;
use url::Url;
use serde::{Deserialize, Serialize};
use notify_rust::Notification;
use reqwest;
use serde_json::Value;

type Result<T> = std::result::Result<T, PulpoError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct NtfyMessage {
    id: String,
    time: u64,
    expires: u64,
    event: String,
    topic: String,
    title: String,
    message: String,
    priority: usize,
    tags: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct NtfyHTTPClient {
    url: Url,
    topics: String,
}

impl NtfyHTTPClient {
    pub fn new(url: Url, topics: String) -> Self {
        NtfyHTTPClient {
            url: url,
            topics: topics,
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

    pub async fn run_loop(&self,poll: u64,notif_sound: &str,notif_icon: &str) -> Result<()> {
        
        // let resp = match reqwest::get(url_to_request.as_str()).await {
        //     Ok(resp) => resp.text().await.unwrap(),
        //     Err(err) => panic!("Error: {}", err)
        // };
        // println!("{}", resp)
        let http_url = self.url.clone();

        loop {
            // attempt to read from the socket
            // let message: Option<GotifyMessage> = match socket.read_message()? {
            let message: Option<NtfyMessage> = match reqwest::get(http_url).await {
                reqwest::Result::Ok(s) => {
                    info!("Message received");
                    Some(serde_json::from_str(&s.text().as_str()))
                }
                reqwest::Result::Err(e) => None,
            };

            // if a message was received create a notification
            if let Some(m) = message {
                let notif = Notification::new()
                    .summary(&m.title)
                    .body(&m.message)
                    .icon(format!("/opt/pulpo/resources/{}",notif_icon).as_str())
                    .show();
                    
                NtfyHTTPClient::play_file(format!("resources/{}",notif_sound).as_str());

                info!("[!] Message received | topic:{} title:{} message:{}",m.topic, m.title,m.message);
                // if the notification fails some how log it but do not kill the process
                // TO DO: Add tracking for the number of failaures and perhaps have it exit after a certain configurable
                // threshhold
                match notif {
                    Ok(_) => info!(
                        "Sent desktop notification: topic:{} title: {} message: {}",
                        m.topic,m.title, m.message
                    ),
                    Err(e) => warn!("Failed to send desktop notification: {}", e),
                }
            }

            thread::sleep(Duration::from_secs(poll));
        }

    }
}