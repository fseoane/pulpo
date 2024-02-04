
use url::Url;

use crate::errors::PulpoError;

type Result<T> = std::result::Result<T, PulpoError>;

/// Takes a url and returns the base url by removing the path
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
    //new_url.join(input)
    new_url.set_query(None);

    Ok(new_url)
}

/// Take a Url and convert it to a websocket url.
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


