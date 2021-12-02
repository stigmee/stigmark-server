// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : Stigmark
//  Version : 0.0-1
// 
//  Stigmee is free software: you can redistribute it and/or modify it
//  under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
// 
//  This program is distributed in the hope that it will be useful, but
//  WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  General Public License for more details.
// 
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
// 

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
