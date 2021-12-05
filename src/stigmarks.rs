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

use serde::{Deserialize};
use rocket::{State, Route};
use rocket_contrib::json::Json;
use rocket_contrib::json;
use rocket::http::Status;
use crate::response::ServerResponse;

#[derive(Deserialize)]
struct StigmarkRequest {
    urls: Vec<String>,
    keys: Vec<String>,
    token: Option<String>,
}

// pub struct StigmarkData {
//     pub user: u32,
//     pub urls: Vec<String>,
//     pub keys: Vec<String>,
// }

// OPTIONS https://stigmark.badro.com/api/v1/stigmarks
#[options("/stigmarks", rank = 1)]
fn stigmarks_options() -> ServerResponse {
    ServerResponse::ok()
}

use stigmarks_sql_rs::sql::SqlStigmarksDB;
// use std::sync::Mutex;

// POST https://stigmark.badro.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<mark>", rank = 1)]
fn stigmarks_post(state: State<SqlStigmarksDB>, mark: Json<StigmarkRequest>) -> ServerResponse {
    let token = &mark.token;
    if let None = token {
        return ServerResponse::error("missing token parameter", Status::BadRequest);
    }
    // todo:
    // if token.unwrap() != "foo" {
    //     return ServerResponse::error("invalid token", Status::Unauthorized);
    // }
    let stigmarks_db = state.inner();
    // todo: note: this user 1 must have been created
    let res = stigmarks_db.add_collection(1, &mark.keys, &mark.urls);
    if let Err(err) = res { 	
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    ServerResponse::json(json!({"collection_id": res.unwrap()}), Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![stigmarks_options, stigmarks_post]
}
