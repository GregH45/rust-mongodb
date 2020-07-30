use bson;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

#[path = "../models/mod.rs"]
mod models;

#[path = "../meta/mod.rs"]
mod meta;


#[post("/user", format = "application/json", data="<user>")]
pub fn create(user: Json<meta::user::Post>) -> JsonValue {
    let model = models::user::Model {
        email: user.email.to_owned(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned()
    };
    
    let document = model.create();
    let result = bson::from_bson::<meta::user::PostResponse>(bson::Bson::Document(document.unwrap()));

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