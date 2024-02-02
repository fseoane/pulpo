use std::fmt;
use url::{ParseError, Url};
use daemonize::Daemonize;
use tungstenite::error::Error as TungError;

#[derive(Debug)]
pub enum PulpoError {
    Json(serde_json::Error),
    IO(std::io::Error),
    Var(std::env::VarError),
    BaseUrl(String),
    UrlParse(ParseError),
    MissingProtocol,
    SchemeError(Url),
    FileNotFound(String),
    Daemonize(daemonize::Error),
    MissingArgs(String),
    UreqResponse(String),
    Tungstenite(TungError),
}

impl fmt::Display for PulpoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            PulpoError::Json(e) => write!(f, "A JSON error occured: {}", e),
            PulpoError::IO(e) => write!(f, "An IO error occured {}", e),
            PulpoError::Var(e) => write!(f, "A environment variable error occured: {}", e),
            PulpoError::BaseUrl(u) => write!(f, "{} cannot be a base URL", u),
            PulpoError::UrlParse(u) => write!(f, "Unable to parse the URL: {}", u),
            PulpoError::MissingProtocol => write!(f, "The URL does not start with HTTP or HTTPS. The connection protocol could not be determined."),
            PulpoError::SchemeError(u) => write!(f, "Could not convert {} into a websocket URL.", u),
            PulpoError::FileNotFound(p) => write!(f, "Could not find {}", p),
            PulpoError::Daemonize(d) => write!(f, "Failed to daemonize: {}", d),
            PulpoError::MissingArgs(e) => write!(f, "{}", e),
            PulpoError::UreqResponse(e) => write!(f, "{}", e),
            PulpoError::Tungstenite(e) => write!(f, "A websocket error occurred: {}", e),
        }
    }
}

impl From<std::io::Error> for PulpoError {
    fn from(err: std::io::Error) -> Self {
        PulpoError::IO(err)
    }
}

impl From<serde_json::Error> for PulpoError {
    fn from(err: serde_json::Error) -> Self {
        PulpoError::Json(err)
    }
}

impl From<std::env::VarError> for PulpoError {
    fn from(err: std::env::VarError) -> Self {
        PulpoError::Var(err)
    }
}

impl From<url::ParseError> for PulpoError {
    fn from(err: url::ParseError) -> Self {
        PulpoError::UrlParse(err)
    }
}

impl From<daemonize::Error> for PulpoError {
    fn from(err: daemonize::Error) -> Self {
        PulpoError::Daemonize(err)
    }
}

impl From<TungError> for PulpoError {
    fn from(err: TungError) -> Self {
        PulpoError::Tungstenite(err)
    }
}
