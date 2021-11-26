use serde::{Deserialize, Serialize};
use rocket::http::{Status};
use rocket_contrib::json::Json;
use std::fmt::Write;
use rocket::Route;
use crate::response::ServerResponse;

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
fn login_options() ->ServerResponse {
    ServerResponse::ok()
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

pub fn routes() -> Vec<Route> {
    routes![login_options, login_post]
}
