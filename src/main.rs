#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mongodb;
use rocket_cors::Cors;

mod controllers;
mod lib;
mod meta;
mod models;
mod config;
mod db;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                controllers::user::create,
                controllers::status::lookup,
                controllers::user::get,
                controllers::user::list,
                controllers::user::login,
            ],
        )
        .register(catchers![controllers::not_found::lookup])
        .attach(config::AppState::manage())
        .attach(cors_fairing())
        .launch();
}
