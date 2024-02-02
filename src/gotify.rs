// gotify api structs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    pub messages: Vec<Message>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
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
