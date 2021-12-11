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

use crate::jwtauth::JwtAuth;
use crate::response::ServerResponse;
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json;
use rocket_contrib::json::Json;
use serde::Deserialize;

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
fn stigmarks_post(
    auth: JwtAuth,
    state: State<SqlStigmarksDB>,
    mark: Json<StigmarkRequest>,
) -> ServerResponse {
    let mut user_id = 0u32;
    if let Some(claims) = auth.claims {
        user_id = claims.uid;
    }
    // We might need token from body
    if user_id > 0 {
        if let Some(token) = &mark.token {
            if let Some(auth) = JwtAuth::new(token) {
                if let Some(claims) = auth.claims {
                    user_id = claims.uid;
                }
            }
        }
    }
    if user_id == 0 {
        println!("access denied");
        return ServerResponse::error("expected token", Status::Forbidden);
    }
    let stigmarks_db = state.inner();
    if let Err(err) = stigmarks_db.get_user_by_id(user_id) {
        println!("could not find user: {}", err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    // todo: check if user is still active
    let res = stigmarks_db.add_collection(user_id, &mark.keys, &mark.urls);
    if let Err(err) = res {
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    ServerResponse::json(json!({"collection_id": res.unwrap()}), Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![stigmarks_options, stigmarks_post]
}
