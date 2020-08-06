use crate::meta;
use crate::models;
use crate::lib::mongo;

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use bson::oid::ObjectId;
use bson::doc;
use mongodb::error::Error;
use serde_json::json;

pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}
impl NewUser<'_> {
    pub fn to_bson(&self) -> bson::document::Document {
        doc! {
            "email": self.email.to_owned(),
            "first_name": self.first_name.to_owned(),
            "last_name": self.last_name.to_owned(),
            "hash": self.hash.to_owned()        }
    }
}

pub fn create(user : meta::user::Post) ->  Option<models::user::User> {
    let params = ScryptParams::new(15, 8, 1);

    let hash = scrypt_simple(&user.password.to_owned(), &params).expect("hash error");
    
    let client = mongo::establish_connection();
    let collection = client.database("test").collection("users");

    let new_user = &NewUser {
        first_name : &user.first_name,
        last_name: &user.last_name,
        email : &user.email,
        hash : &hash,
    };

    let inserted_id = collection.insert_one(new_user.to_bson(), None);

    if inserted_id.is_ok() {
        let result = match collection.find_one(Some(doc! { "_id" : inserted_id.unwrap().inserted_id }), None) {
            Ok(user) => match user {
                Some(a) => Some(a),
                None => None
            },
            Err(_) => None
        };

        let user = match result {
            Some(result) => models::user::from_document(result),
            None => None
        };
        return user;

    } else {
        return None
    }
}

pub fn find_one(user_id : String) -> Option<models::user::User> {
    
    let client = mongo::establish_connection();
    let collection = client.database("test").collection("users");

    let id = ObjectId::with_string(&user_id).unwrap();

    let result = match collection.find_one(Some(doc! { "_id" : id }), None) {
        Ok(user) => match user {
            Some(a) => Some(a),
            None => None
        },
        Err(_) => None
    };

    let user = match result {
        Some(result) => models::user::from_document(result),
        None => None
    };
    return user;
}

pub fn find(options: bson::Document) ->  Vec<models::user::User> {
    let client = mongo::establish_connection();
    let collection = client.database("test").collection("users");

    let cursor = match collection.find(Some(options), None) {
        Ok(result) => result,
        Err(_) => panic!("fail to create cursor")
    };
    
    let mut vec = Vec::new();

    for result in cursor {
        let doc = result.expect("msg");
        let s_doc = models::user::from_document(doc).unwrap();
        vec.push(s_doc);
    }
    println!("toto");
    
    vec
}

pub fn login(user : meta::user::PostAuth) -> Option<models::user::User> {
    let client = mongo::establish_connection();
    let collection = client.database("test").collection("users");

    let filter = doc! { "email": user.email.to_owned()};
    let user_d = collection.find_one(Some(filter), None).ok().expect("User Not Found");
    
    if let Some(a) = &user_d {
        if let Some(hash) = &a.get("hash") {
            if let Some(str_hash) = &hash.as_str() {
                let password_matches = scrypt_check(&user.password, &str_hash)
                    .is_ok();

                if password_matches {
                    return models::user::from_document(user_d.unwrap())
                } else {
                    eprintln!(
                        "login attempt for '{}' failed: password doesn't match",
                        user.email.to_owned()
                    );
                    return None
                }
            }

        }
    
    }
    None
    // println!("{:?}", hash);
}