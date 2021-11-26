#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// extern crate bcrypt;

// rocket stuff
use rocket::config::{Config, Environment};

// thread stuff
use std::sync::mpsc;
use std::thread;

// crate modules
mod response;
mod basicauth;

mod files;
mod login;

mod stigmarks;
use stigmarks::StigmarkData;

mod stigmers;
use stigmers::StigmerService;

mod database;
use database::save_stigmarks_service;

mod cors;
use cors::CORS;

fn main() {
    // start service thread
    let (tx, rx): (
        mpsc::SyncSender<StigmarkData>,
        mpsc::Receiver<StigmarkData>,
    ) = mpsc::sync_channel(256);
    thread::spawn(move || save_stigmarks_service(rx));

    // start the user manager
    // TODO ----------------------------------------------------
    let svc = StigmerService::new("/data/stigmers.json");
    let user_id = svc.find_user_by_email(String::from("zexigh@gmail.com"));
    println!("found default user at {}", user_id);
    // TODO ----------------------------------------------------

    // start the web service
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    let mut api_routes = stigmarks::routes();
    api_routes.append(&mut login::routes());

    rocket::custom(config)
        .manage(tx)
        .attach(CORS)
        .mount("/api/v1", api_routes)
        .mount("/", files::routes())
        .launch();
}
