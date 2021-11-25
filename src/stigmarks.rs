use std::sync::mpsc;
use serde::{Deserialize};
use rocket::{State, Route};
use rocket_contrib::json::Json;
use rocket::http::Status;

#[derive(Deserialize)]
struct StigmarkRequest {
    urls: Vec<String>,
    keys: Vec<String>,
    token: Option<String>,
}

pub struct StigmarkData {
    pub user: u32,
    pub urls: Vec<String>,
    pub keys: Vec<String>,
}

// OPTIONS https://stigmark.badro.com/api/v1/stigmarks
#[options("/stigmarks")]
fn stigmarks_options() {
    println!("stigmarks: OPTIONS /api/v1/stigmarks");
}

// POST https://stigmark.badro.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<mark>")]
fn stigmarks_post(tx: State<mpsc::SyncSender<StigmarkData>>, mark: Json<StigmarkRequest>) -> Status {
    match &mark.token {
        Some(token) => {
            if token != "foo" {
                return Status::Unauthorized;
            }
        },
        None => {
            return Status::Unauthorized;
        }
    }
    tx.send(StigmarkData {
        user: 3,
        urls: mark.urls.clone(), // todo: remove this clone()
        keys: mark.keys.clone(), // todo: remove this clone()
    }).unwrap();
    Status::Ok
}

pub fn stigmarks_routes() -> Vec<Route> {
    routes![stigmarks_options, stigmarks_post]
}

// #[delete("/stigmarks", format = "json", data = "<mark>")]
// fn stigmarks_unmark(mark: Json<Stigmark>) {
//     println!("stigmarks: DELETE /api/v1/stigmarks: {}", mark.url);
// }
