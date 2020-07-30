#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mongodb;

use bson;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

mod lib;
mod models;

type ID = String;

#[get("/")]
pub fn create() -> JsonValue {
    let model = models::user::Model {
        email: "gregoire.harba@gmail.com".to_owned(),
        first_name: "Grégoire".to_owned(),
        last_name: "Harba".to_owned()
    };
    
    let document = model.create();
    let result = bson::from_bson::<PostResponse>(bson::Bson::Document(document.unwrap()));
    println!("{:?}", result);

    match result {
        Ok(model) => {
            json!({
                "code": 201,
                "success": true,
                "data": model,
                "error": "",
            })
        },
        Err (_e) => {
            json!({
                "code": 412,
                "success": false,
                "data": {},
                "error": "An error has occured",
            })
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![create]).launch();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
  pub _id: bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}