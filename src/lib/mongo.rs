use mongodb;
use mongodb::Client;
use mongodb::error::Result;
use mongodb::options::{ClientOptions, StreamAddress};

use dotenv;

const DEFAULT_MONGO_ADDRESS: &'static str = "127.0.0.1";
const DEFAULT_MONGO_PORT: u16 = 27017;

pub fn establish_connection() -> Result<Client> {
    let database_url = match dotenv::var("MONGO_ADDRESS") {
        Ok(value) => value,
        Err(_) => DEFAULT_MONGO_ADDRESS.to_string(),
    };

    let database_port = match dotenv::var("MONGO_PORT") {
        Ok(value) => value.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_MONGO_PORT,
    };

    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: database_url,
            port: Some(database_port),
        }])
        .build();

    let client = Client::with_options(options);
    
    client
}
