#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate bcrypt;

// rocket stuff
use rocket::config::{Config, Environment};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::{Request, Response};

// json, stigmark data
use std::path::{Path, PathBuf};
use std::vec::Vec;

// thread stuff
use std::sync::mpsc;
use std::thread;

// this is required by rocket to add cors headers
pub struct CORS;
impl Fairing for CORS {
    fn info(&self) -> Info {
        println!("Fairing::on_response");
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        println!("Fairing::on_response");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// GET https://stigmark.badro.com/
#[get("/", rank = 1)]
fn slash() -> Result<NamedFile, NotFound<String>> {
    println!("stigmarks: GET /");
    let path = Path::new("www/index.htm");
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

// GET https://stigmark.badro.com/*
#[get("/<file..>", rank = 2)]
fn others(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    println!("stigmarks: GET {:?}", file);
    let path = Path::new("www/").join(file);
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

// #[get("/stigmarks")]
// fn stigmarks_enum() -> String {
//     println!("stigmarks: GET /api/v1/stigmarks");
//     let r = format!("stigmarks_emum");
//     r
// }

mod login;
use login::login_routes;

mod stigmarks;
use stigmarks::StigmarkData;
use stigmarks::stigmarks_routes;

mod stigmers;
use stigmers::StigmerService;

mod database;
use database::save_stigmarks_service;

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

    let mut api_routes = stigmarks_routes();
    api_routes.append(&mut login_routes());

    rocket::custom(config)
        .manage(tx)
        .attach(CORS)
        .mount("/api/v1", api_routes)
        .mount("/", routes![slash, others])
        .launch();
}
