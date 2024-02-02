use std::convert::TryFrom;

use ureq;
use url::Url;

use crate::args::Args;
use crate::errors::GdndError;
use crate::gotify::Client;
use crate::helpers::base_url;

type Result<T> = std::result::Result<T, GdndError>;

pub struct UserAuth {
    pub client: String,
    pub username: String,
    pub password: String,
    pub url: Url,
}

impl UserAuth {
    fn new(username: &str, password: &str, client: &str, url: Url) -> UserAuth {
        UserAuth {
            username: username.to_string(),
            password: password.to_string(),
            client: client.to_string(),
            url,
        }
    }

    pub fn authenticate(&self) -> Result<Client> {
        // set url path
        // the url is cloned so the struct does not have to be mutable
        let mut url = self.url.clone();
        url.set_path("client");

        // create the client using the provided username and auth
        let resp = ureq::post(url.as_str())
            .query("name", self.client.as_str())
            .auth(&self.username, &self.password)
            .call();

        // if created succesfully deserialize the resp into a Client struct
        let cli = if resp.ok() {
            resp.into_json_deserialize::<Client>()?
        } else {
            let err_msg = format!("Authentication request failed: {}", resp.status_line());
            return Err(GdndError::UreqResponse(err_msg));
        };

        Ok(cli)
    }
}

impl TryFrom<Args> for UserAuth {
    type Error = GdndError;

    fn try_from(value: Args) -> Result<UserAuth> {
        let url = base_url(&value.url)?;

        let client = match value.client {
            Some(c) => c,
            None => {
                let err_msg =
                    "Client name is required when authenticating via username and password."
                        .to_string();
                return Err(GdndError::MissingArgs(err_msg));
            }
        };

        let username = match value.username {
            Some(u) => u,
            None => {
                let err_msg = "Username is required when authenticating via username and password."
                    .to_string();
                return Err(GdndError::MissingArgs(err_msg));
            }
        };

        let password = match value.password {
            Some(p) => p,
            None => {
                let err_msg = "Password is required when authenticating via username and password."
                    .to_string();
                return Err(GdndError::MissingArgs(err_msg));
            }
        };

        Ok(UserAuth::new(&username, &password, &client, url))
    }
}
