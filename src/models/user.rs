use bson::{doc, document::Document};
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};

use crate::{meta::user, lib::auth::Auth};

//Model from database
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: bson::oid::ObjectId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub hash: String,
}


impl User {
    //Cast struct to BSON document
    pub fn to_bson(&self) -> Document {
        doc! {
            "_id" : self.email.to_owned(),
            "email": self.email.to_owned(),
            "first_name": self.first_name.to_owned(),
            "last_name": self.last_name.to_owned(),
            "hash": self.hash.to_owned()        }
    }
    
    //Cast a User to a Struct of User with generated Token
    pub fn to_user_auth(&self, secret: &[u8]) -> user::UserAuth {
        let exp = Utc::now() + Duration::days(60); 
        let token = Auth {
            id: self._id.clone(),
            username: self.email.clone(),
            exp: exp.timestamp(),
        }.token(secret);

        user::UserAuth {
            first_name: self.first_name.to_owned(),
            last_name: self.last_name.to_owned(),
            email: self.email.to_owned(),
            token: token.to_owned(),
        }
    }
}   

pub fn from_document(document : bson::document::Document) -> Option<User> {
    Some(User {
        _id: document.get("_id").unwrap().as_object_id().unwrap().to_owned(),
        email: document.get("email").unwrap().as_str().unwrap().to_owned(),
        first_name: document.get("first_name").unwrap().as_str().unwrap().to_owned(),
        last_name: document.get("last_name").unwrap().as_str().unwrap().to_owned(),
        hash: document.get("hash").unwrap().as_str().unwrap().to_owned(),
    })
}