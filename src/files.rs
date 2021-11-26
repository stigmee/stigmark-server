use std::path::{Path, PathBuf};
use rocket::Route;

use crate::response::ServerResponse;
use crate::basicauth::BasicAuth;

// GET https://stigmark.badro.com/
#[get("/", rank = 2)]
fn files_slash(auth: BasicAuth) -> ServerResponse {
    println!("stigmarks: '{}' GET /", auth.name);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/index.htm");
    ServerResponse::file(&path)
}

// GET https://stigmark.badro.com/*
#[get("/<file..>", rank = 3)]
fn files_others(auth: BasicAuth, file: PathBuf) -> ServerResponse {
    println!("stigmarks: '{}' GET {:?}", auth.name, file);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/").join(file);
    ServerResponse::file(&path)
}

pub fn routes() -> Vec<Route> {
    routes![files_slash, files_others]
}
