#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// use rocket::http::Method;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
// use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Guard, Responder};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::vec::Vec;

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

#[get("/", rank = 1)]
fn slash() -> Result<NamedFile, NotFound<String>> {
    println!("stigmarks: GET /");
    let path = Path::new("www/index.htm");
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

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

#[options("/stigmarks")]
fn stigmarks_options() {
    println!("stigmarks: OPTIONS /api/v1/stigmarks");
}

#[post("/stigmarks", format = "json", data = "<mark>")]
fn stigmarks_mark(mark: Json<Stigmark>) {
    println!("stigmarks: POST /api/v1/stigmarks: {:?} {:?}", mark.urls, mark.keys);
}

// #[delete("/stigmarks", format = "json", data = "<mark>")]
// fn stigmarks_unmark(mark: Json<Stigmark>) {
//     println!("stigmarks: DELETE /api/v1/stigmarks: {}", mark.url);
// }

// main

fn main() {
    rocket::ignite()
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
