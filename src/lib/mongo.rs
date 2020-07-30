use std;
use std::env;

use mongodb;
use mongodb::Client;
use mongodb::error::Result;
use mongodb::options::{ClientOptions, StreamAddress};

const DEFAULT_MONGO_ADDRESS: &'static str = "127.0.0.1";

pub fn establish_connection() -> Result<Client> {
    let database_url = match env::var("MONGO_ADDRESS") {
        Ok(value) => value,
        Err(_) => DEFAULT_MONGO_ADDRESS.to_string(),
    };

    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: database_url,
            port: Some(27017),
        }])
        .build();

    let client = Client::with_options(options);
    
    client
}
