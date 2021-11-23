#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// rocket stuff
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, State};

// json, stigmark data
use serde::{Serialize, Deserialize};
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

#[derive(Deserialize)]
struct Stigmark {
    urls: Vec<String>,
    keys: Vec<String>,
}

// OPTIONS https://stigmark.badro.com/api/v1/stigmarks
#[options("/stigmarks")]
fn stigmarks_options() {
    println!("stigmarks: OPTIONS /api/v1/stigmarks");
}

// POST https://stigmark.badro.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<mark>")]
fn stigmarks_mark(tx: State<mpsc::SyncSender<Stigmark>>, mark: Json<Stigmark>) {
    tx.send(mark.0).unwrap()
}

// #[delete("/stigmarks", format = "json", data = "<mark>")]
// fn stigmarks_unmark(mark: Json<Stigmark>) {
//     println!("stigmarks: DELETE /api/v1/stigmarks: {}", mark.url);
// }

#[derive(Serialize, Deserialize)]
struct StigmarkDB {
    groups: Vec<StigmarkGroup>,
}

#[derive(Serialize, Deserialize)]
struct StigmarkGroup {
    gid: u32,
    urls: Vec<StigmarkURL>,
    stigmarks: Vec<StigmarkMarks>,
}

#[derive(Serialize, Deserialize)]
struct StigmarkURL {
    uid: u32,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct StigmarkMarks {
    urls: Vec<u32>,
    keywords: Vec<String>,
}

// handles json database
fn save_stigmarks_service(rx: mpsc::Receiver<Stigmark>) {
    loop {
        let mark = rx.recv().unwrap();
        println!("stigmarks: POST /api/v1/stigmarks: {:?} {:?}", mark.urls, mark.keys);
    }
}

fn main() {
    // start service thread
    let (tx, rx): (mpsc::SyncSender<Stigmark>, mpsc::Receiver<Stigmark>) = mpsc::sync_channel(256);
    thread::spawn(move || { save_stigmarks_service(rx) });

    rocket::ignite()
        .manage(tx)
        .attach(CORS)
        .mount(
            "/api/v1",
            routes![
                stigmarks_options,
                stigmarks_mark,
                // stigmarks_unmark,
            ],
        )
        .mount("/", routes![slash, others])
        .launch();
}
