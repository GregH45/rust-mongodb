use bson; 
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
  pub _id: bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
  pub _id: bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostAuth {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct UserAuth {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}