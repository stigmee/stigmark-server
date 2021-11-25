#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate bcrypt;

// rocket stuff
use rocket::config::{Config, Environment};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::{Request, Response, State};
use rocket_contrib::json::Json;

// json, stigmark data
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::vec::Vec;

// thread stuff
use std::sync::mpsc;
use std::thread;

// file stuff
use std::fs::File;
use std::io::BufReader;

// byte -> hex formating
use std::fmt::Write;

// backup stuff
use chrono::{Datelike, Timelike, Utc};
const MAX_UPDATES_BEFORE_SAVING: u32 = 5;

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
struct LoginRequest {
    email: String,
    passwd: String,
}

#[derive(Serialize)]
struct LoginResult {
    token: String,
}

#[options("/login")]
fn login_options() {
    println!("stigmarks: OPTIONS /api/v1/login");
}

#[post("/login", format = "json", data = "<req>")]
fn login_post(req: Json<LoginRequest>) -> Status {
    let passwd = &req.passwd;
    let hash = bcrypt::hash(passwd, 6).unwrap();
    let mut hash_string = String::new();
    for byte in hash.bytes() {
        write!(&mut hash_string, "{:X}", byte).expect("Unable to write");
    }
    println!("user: {} -> {}", req.email, hash_string);
    Status::Ok
}

#[derive(Deserialize)]
struct StigmarkRequest {
    urls: Vec<String>,
    keys: Vec<String>,
    token: String,
}

// OPTIONS https://stigmark.badro.com/api/v1/stigmarks
#[options("/stigmarks")]
fn stigmarks_options() {
    println!("stigmarks: OPTIONS /api/v1/stigmarks");
}

// POST https://stigmark.badro.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<mark>")]
fn stigmarks_post(
    tx: State<mpsc::SyncSender<StigmarkRequest>>,
    mark: Json<StigmarkRequest>,
) -> Status {
    if mark.token != "foo" {
        return Status::Unauthorized;
    }
    tx.send(mark.0).unwrap();
    Status::Ok
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

fn read_db_from_json(name: &str) -> Result<StigmarkDB, String> {
    match File::open(name) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let res: Result<StigmarkDB, serde_json::Error> = serde_json::from_reader(reader);
            match res {
                Ok(db) => Ok(db),
                Err(err) => Err(format!("{}", err)),
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}

fn write_db_to_json(name: &str, db: &StigmarkDB) {
    let file = File::create(name).unwrap();
    serde_json::to_writer(file, &db).unwrap();
}

const STIGMARK_FILE_NAME: &str = "data/stigmarks.json";

// handles json database
fn save_stigmarks_service(rx: mpsc::Receiver<StigmarkRequest>) {
    let mut stigmark_db = match read_db_from_json(STIGMARK_FILE_NAME) {
        Ok(stigmark_db) => stigmark_db,
        Err(_) => {
            let group0 = StigmarkGroup {
                gid: 1,
                urls: vec![],
                stigmarks: vec![],
            };
            StigmarkDB {
                groups: vec![group0],
            }
        }
    };

    let mut updates_before_backup = MAX_UPDATES_BEFORE_SAVING;
    loop {
        let mark = rx.recv().unwrap();

        updates_before_backup -= 1;
        if updates_before_backup == 0 {
            updates_before_backup = MAX_UPDATES_BEFORE_SAVING;
            let now = Utc::now();
            let backup_name = format!(
                "data/stigmarks-{}-{}-{}-{}-{}-{}.json",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            write_db_to_json(backup_name.as_str(), &stigmark_db);
        }

        let mut urls = vec![]; // urls uids
        for u in mark.urls {
            // look if url already in db
            let (mut uid, mut found) = (0, false);
            for ku in &stigmark_db.groups[0].urls {
                if u == ku.url {
                    uid = ku.uid;
                    found = true;
                };
            }
            // if not in db, add it
            if !found {
                uid = stigmark_db.groups[0].urls.len() as u32;
                stigmark_db.groups[0].urls.push(StigmarkURL {
                    uid: uid,
                    url: u.clone(),
                });
                println!("stigmark: added url {}:{}", uid, u);
            }
            urls.push(uid);
        }

        // add mark to db
        stigmark_db.groups[0].stigmarks.push(StigmarkMarks {
            urls: urls,
            keywords: mark.keys,
        });

        write_db_to_json(STIGMARK_FILE_NAME, &stigmark_db);
    }
}

fn main() {
    // start service thread
    let (tx, rx): (
        mpsc::SyncSender<StigmarkRequest>,
        mpsc::Receiver<StigmarkRequest>,
    ) = mpsc::sync_channel(256);
    thread::spawn(move || save_stigmarks_service(rx));

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(tx)
        .attach(CORS)
        .mount(
            "/api/v1",
            routes![login_options, stigmarks_options, login_post, stigmarks_post],
        )
        .mount("/", routes![slash, others])
        .launch();
}
