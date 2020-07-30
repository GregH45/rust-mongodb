use bson::doc;
use tokio;
// use bson::oid::ObjectId;
// use std;
// use std::io;

use mongodb::error::Error;

#[path = "../lib/mod.rs"]
mod lib;

#[derive(Debug)]
pub struct Model {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl Model {
    pub fn to_bson(&self) -> bson::document::Document {
        doc! {
            "email": self.email.to_owned(),
            "first_name": self.first_name.to_owned(),
            "last_name": self.last_name.to_owned(),
        }
    }

    #[tokio::main]
    pub async fn create(&self) ->  Result<bson::document::Document, Error> {
        let client = lib::mongo::establish_connection();
        let collection = client.unwrap().database("test").collection("users");
        collection.insert_one(self.to_bson(), None).await.unwrap();
        
        let inserted_one = collection.find_one(Some(self.to_bson()), None).await.unwrap().unwrap();
        Ok(inserted_one)
    }

    // pub fn find_one(user_id: String) -> Result<bson::document::Document, io::Error> {
    //     let client = lib::mongo::establish_connection();
    //     let collection = client.unwrap().database("test").collection("users");
    
    //     let id = ObjectId::with_string(&user_id).unwrap();

    //     let aze = doc! {
    //         "_id" => id
    //     };

    //     let response_document = collection.find_one(Some(aze), None)
    //         .ok().expect("Failed to execute find.");
          
    //     Ok(response_document)
    // }
}
