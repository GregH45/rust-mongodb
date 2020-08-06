use bson;
use bson::doc;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket::State;
use crate::config::AppState;

use crate::meta;
use crate::models;
use crate::lib::auth::Auth;
use crate::db;

type ID = String;

#[post("/user", format = "application/json", data = "<new_user>")]
pub fn create(new_user: Json<meta::user::Post>, state: State<AppState>) -> JsonValue {
    let new_user = new_user.into_inner();

    let created_user = db::user::create(new_user);
    match created_user {
        Some(model) => json!({
            "code": 201,
            "success": true,
            "data": model.to_user_auth(&state.secret),
            "error": "",
        }),
        None => json!({
            "code": 412,
            "success": false,
            "data": {},
            "error": "An error has occured",
        }),
    }
}

#[get("/user/<id>", format = "application/json")]
pub fn get(id: ID) -> JsonValue {
    let document = db::user::find_one(id.to_owned());

    match document {
        Some(user) => json!({
            "code": 200,
            "success": true,
            "data": user,
            "error": ""
        }),
        None => json!({
          "code": 200,
          "success": true,
          "data": {},
          "error": ""
        }),
    }
}

#[get("/user", format = "application/json")]
pub fn list() -> JsonValue {
    let documents = db::user::find(doc! {});
    
    json!({
        "code" : 200,
        "success": true,
        "data": documents
    })
}

#[post("/user/login", format="application/json", data="<user>")]
pub fn login(user : Json<meta::user::PostAuth>, state: State<AppState>) -> JsonValue {
    let user_doc = db::user::login(user.into_inner()).unwrap();

    json!({
        "code" : 200,
        "success": true,
        "data": user_doc.to_user_auth(&state.secret)
    })
    // models::user::login(user.0)
    //     .map(|user|json!({"data": user.to_user_auth("azeazeazeqsd")}));
}