use std::env;

use anyhow::{ensure, Result};
use reqwest::{blocking::Client, StatusCode, Url};

const STRAVA_URL: &str = "https://www.strava.com";

#[derive(Debug, Default)]
pub struct AuthData {
    pub client_secret: String,
    pub client_id: String,
    pub refresh_token: String,
    pub access_token: String,
}

impl AuthData {
    pub fn new(client: &Client) -> Result<Self> {
        let client_id = env::var("CLIENT_ID")?;
        let client_secret = env::var("CLIENT_SECRET")?;

        let url = Url::parse_with_params(
            &format!("{}/oauth/token", STRAVA_URL),
            &[
                ("client_id", &client_id),
                ("client_secret", &client_secret),
                ("code", &env::var("OAUTH_CODE")?),
                ("grant_underscore", &String::from("authorization_code")),
            ],
        )?;
        let resp = client.post(url).send()?;
        ensure!(
            resp.status() == StatusCode::OK,
            "Response didn't have a status code of 200"
        );
        let data = resp.json::<response::InitialCode>()?;
        Ok(AuthData {
            client_secret,
            client_id,
            refresh_token: data.refresh_token,
            access_token: data.access_token,
        })
    }
}

mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct InitialCode {
        pub refresh_token: String,
        pub access_token: String,
    }
}
