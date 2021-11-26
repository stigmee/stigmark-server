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


struct BasicAuth {
    name: String,
    pass: String,
}

use base64;

impl BasicAuth {
    /// Creates a new [BasicAuth] struct/request guard from a given plaintext
    /// http auth header or returns a [Option::None] if invalid
    pub fn new<T: Into<String>>(auth_header: T) -> Option<Self> {
        let key = auth_header.into();
        if key.len() < 7 || &key[..6] != "Basic " {
            return None;
        }

        let decoded = match base64::decode(&key[6..]) {
            Ok(bytes) => String::from_utf8(bytes).unwrap(),
            Err(err) => format!("failed to decode {}", err),
        };

        let (name, pass) = decoded.split_once(":").unwrap();

        Some(Self {
            name: String::from(name),
            pass: String::from(pass),
        })
    }
}

// requests: look for authorization header (if any)

use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

#[derive(Debug)]
enum LoginError {
    InvalidUserPassCombination,
    BadCount,
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("BasicAuth::FromRequest keys={}", keys.join(","));
        match keys.len() {
            0 => Outcome::Success(BasicAuth{name: String::from(""), pass: String::from("")}),
            1 => match BasicAuth::new(keys[0]) {
                Some(auth_header) => Outcome::Success(auth_header),
                None => Outcome::Failure((Status::BadRequest, LoginError::InvalidUserPassCombination)),
            },
            _ => Outcome::Failure((Status::BadRequest, LoginError::BadCount)),
        }
        // Outcome::Failure((Status::Unauthorized, LoginError::InvalidUserPassCombination))

        // match (username, password) {
        //     (Some(u), Some(p)) => {
        //         Outcome::Success(BasicUser{name: String::from(u), pass: String::from(p)})
        //     }
        //     _ => Outcome::Failure((Status::Unauthorized, LoginError::InvalidData))
        // }
    }
}

// responses : can return either 

#[derive(Debug)]
enum ServerResponse {
    File(NamedFile),
    Json(JsonValue),
    Error(Status),
    BasicAuth(),
}

impl ServerResponse {
    pub fn file(file: &Path) -> Self {
        match NamedFile::open(file) {
            Ok(file) => Self::File(file),
            Err(_) => Self::Error(Status::NotFound),
        }
    }

    #[allow(dead_code)]
    pub fn json(json: JsonValue) -> Self {
        Self::Json(json)
    }

    pub fn basic_auth() -> Self {
        Self::BasicAuth()
    }
}

use rocket::response::Responder;
use rocket::response;
use rocket_contrib::json::JsonValue;

impl<'r>  Responder<'r> for ServerResponse {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        let mut res = Response::build();
        match self {
            Self::File(file) => res.sized_body(file),
            Self::Error(status) => res.status(status),
            Self::BasicAuth() => {
                res
                    .raw_header("WWW-Authenticate", "Basic realm=\"User Visible Realm\", charset=\"UTF-8\"")
                    .status(Status::Unauthorized)
            }
            _ => res.status(Status::NotImplemented),
        }.ok()
    }
}

// GET https://stigmark.badro.com/
#[get("/", rank = 1)]
fn slash(auth: BasicAuth) -> ServerResponse {
    println!("stigmarks: '{}' GET /", auth.name);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/index.htm");
    ServerResponse::file(&path)
}

// GET https://stigmark.badro.com/*
#[get("/<file..>", rank = 2)]
fn others(auth: BasicAuth, file: PathBuf) -> ServerResponse {
    println!("stigmarks: '{}' GET {:?}", auth.name, file);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/").join(file);
    ServerResponse::file(&path)
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
