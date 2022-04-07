use dotenv::dotenv;
use reqwest::blocking::Client;
use strava::AuthData;

mod strava;

fn main() {
    dotenv().ok();
    let client = Client::new();
    let auth_data = AuthData::new(&client).expect("Failed to load authdata");
    dbg!(auth_data);
}
