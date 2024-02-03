use std::env::var;

use url::Url;

use crate::errors::PulpoError;

type Result<T> = std::result::Result<T, PulpoError>;

// Takes a url and returns the base url by removing the path
pub fn base_url(url: &Url) -> Result<Url> {
    //let full_url = url.clone().into_string();
    let full_url = String::from(url.clone());
    let mut new_url = url.clone();
    match new_url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => return Err(PulpoError::BaseUrl(full_url)),
    }

    new_url.set_query(None);

    Ok(new_url)
}

// Take a Url and convert it to a websocket url.
pub fn to_websocket(url: Url) -> Result<Url> {
    const WS: &str = "ws";
    const WSS: &str = "wss";

    let mut ws_url = url.clone();
    match url.scheme() {
        "https" => {
            if let Err(_) = ws_url.set_scheme(WSS) {
                return Err(PulpoError::SchemeError(url));
            };
        }
        "http" => {
            if let Err(_) = ws_url.set_scheme(WS) {
                return Err(PulpoError::SchemeError(url));
            };
        }
        _ => return Err(PulpoError::MissingProtocol),
    }

    Ok(ws_url)
}

// Take a Url and convert it to a websocket url.
pub fn to_ntfyurl(url: Url,topics: String) -> Result<Url> {

    let mut ws_url = url.clone();
    ws_url.set_path(format!("{}/{}",topics.as_str(),"json").as_str());

    Ok(ws_url)
}

// Create the path for writing the json file for the client cache
//
// The default path is $HOME/.cache/gdnd
// If the HOME environment variable is not set the will fail
// pub fn get_cache_path() -> Result<String> {
//     let home = var("HOME")?;
//     let cache_path = format!("{}/.cache/gdnd", home);
//     Ok(cache_path)
// }
