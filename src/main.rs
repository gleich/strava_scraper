use dotenv::dotenv;
use reqwest::blocking::Client;
use strava::AuthData;

use crate::strava::Activity;

mod strava;

fn main() {
    dotenv().ok();
    let client = Client::new();
    let mut auth_data = AuthData::new(&client).expect("Failed to load authdata");
    dbg!(&auth_data);
    auth_data
        .request_new_token(&client)
        .expect("Failed to request new token");
    dbg!(&auth_data);

    let activites =
        Activity::fetch_recent(&auth_data, &client).expect("Failed to fetch recent activates");
    dbg!(activites);
}
