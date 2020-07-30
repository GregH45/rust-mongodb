#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mongodb;

mod controllers;
mod lib;
mod models;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![controllers::user::create, controllers::status::lookup],
        )
        .register(catchers![controllers::not_found::lookup])
        .launch();
}
