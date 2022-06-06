use std::env;

use anyhow::{ensure, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::{blocking::Client, StatusCode, Url};
use serde::Deserialize;

const STRAVA_URL: &str = "https://www.strava.com";

#[derive(Debug)]
pub struct AuthData {
    pub client_secret: String,
    pub client_id: String,
    pub refresh_token: String,
    pub access_token: String,
    pub token_expire_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    pub elapsed_time: f32,
    #[serde(rename(deserialize = "type"))]
    pub activity_type: String,
    pub average_heartrate: Option<f32>,
    pub max_heartrate: Option<f32>,
    pub has_heartrate: bool,
    pub average_speed: f32,
    pub max_speed: f32,
    pub moving_time: f32,
    pub name: String,
    pub pr_count: f32,
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
        dbg!(resp.status());
        ensure!(
            resp.status() == StatusCode::OK,
            "Response didn't have a status code of 200"
        );
        let data = resp.json::<response::Token>()?;
        Ok(AuthData {
            client_secret,
            client_id,
            refresh_token: data.refresh_token,
            access_token: data.access_token,
            token_expire_time: DateTime::from_utc(
                NaiveDateTime::from_timestamp(data.expires_at.into(), 0),
                Utc,
            ),
        })
    }

    pub fn request_new_token(&mut self, client: &Client) -> Result<()> {
        let url = Url::parse_with_params(
            &format!("{}/api/v3/oauth/token", STRAVA_URL),
            &[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("grant_type", &String::from("refresh_token")),
                ("refresh_token", &self.refresh_token),
            ],
        )?;
        let resp = client.post(url).send()?;
        let data = resp.json::<response::Token>()?;
        self.refresh_token = data.refresh_token;
        self.token_expire_time = DateTime::from_utc(
            NaiveDateTime::from_timestamp(data.expires_at.into(), 0),
            Utc,
        );
        self.access_token = data.access_token;
        Ok(())
    }
}

impl Activity {
    pub fn fetch_recent(auth_data: &AuthData, client: &Client) -> Result<Vec<Self>> {
        let resp = client
            .get(format!("{}/api/v3/athlete/activities", STRAVA_URL))
            .bearer_auth(&auth_data.access_token)
            .send()?;
        dbg!(resp.status());
        ensure!(
            resp.status() == StatusCode::OK,
            "Response didn't have status code of 200"
        );
        let data = resp.json::<Vec<Activity>>()?;
        Ok(data)
    }
}

mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Token {
        pub refresh_token: String,
        pub access_token: String,
        pub expires_at: u32,
    }
}
