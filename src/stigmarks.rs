use std::sync::mpsc;
use serde::{Deserialize};
use rocket::{State, Route};
use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::response::ServerResponse;

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
#[options("/stigmarks", rank = 1)]
fn stigmarks_options() -> ServerResponse {
    ServerResponse::ok()
}

// POST https://stigmark.badro.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<mark>", rank = 1)]
fn stigmarks_post(tx: State<mpsc::SyncSender<StigmarkData>>, mark: Json<StigmarkRequest>) -> Status {
    match &mark.token {
        Some(_token) => {
            // if token != "foo" {
            //     return Status::Unauthorized;
            // }
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

pub fn routes() -> Vec<Route> {
    routes![stigmarks_options, stigmarks_post]
}
