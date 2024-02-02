use std::fmt;
use url::{ParseError, Url};
use daemonize::DaemonizeError;
use tungstenite::error::Error as TungError;

#[derive(Debug)]
pub enum GdndError {
    Json(serde_json::Error),
    IO(std::io::Error),
    Var(std::env::VarError),
    BaseUrl(String),
    UrlParse(ParseError),
    MissingProtocol,
    SchemeError(Url),
    FileNotFound(String),
    Daemonize(DaemonizeError),
    MissingArgs(String),
    UreqResponse(String),
    Tungstenite(TungError),
}

impl fmt::Display for GdndError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            GdndError::Json(e) => write!(f, "A JSON error occured: {}", e),
            GdndError::IO(e) => write!(f, "An IO error occured {}", e),
            GdndError::Var(e) => write!(f, "A environment variable error occured: {}", e),
            GdndError::BaseUrl(u) => write!(f, "{} cannot be a base URL", u),
            GdndError::UrlParse(u) => write!(f, "Unable to parse the URL: {}", u),
            GdndError::MissingProtocol => write!(f, "The URL does not start with HTTP or HTTPS. The connection protocol could not be determined."),
            GdndError::SchemeError(u) => write!(f, "Could not convert {} into a websocket URL.", u),
            GdndError::FileNotFound(p) => write!(f, "Could not find {}", p),
            GdndError::Daemonize(d) => write!(f, "Failed to daemonize: {}", d),
            GdndError::MissingArgs(e) => write!(f, "{}", e),
            GdndError::UreqResponse(e) => write!(f, "{}", e),
            GdndError::Tungstenite(e) => write!(f, "A websocket error occurred: {}", e),
        }
    }
}

impl From<std::io::Error> for GdndError {
    fn from(err: std::io::Error) -> Self {
        GdndError::IO(err)
    }
}

impl From<serde_json::Error> for GdndError {
    fn from(err: serde_json::Error) -> Self {
        GdndError::Json(err)
    }
}

impl From<std::env::VarError> for GdndError {
    fn from(err: std::env::VarError) -> Self {
        GdndError::Var(err)
    }
}

impl From<url::ParseError> for GdndError {
    fn from(err: url::ParseError) -> Self {
        GdndError::UrlParse(err)
    }
}

impl From<daemonize::DaemonizeError> for GdndError {
    fn from(err: daemonize::DaemonizeError) -> Self {
        GdndError::Daemonize(err)
    }
}

impl From<TungError> for GdndError {
    fn from(err: TungError) -> Self {
        GdndError::Tungstenite(err)
    }
}
