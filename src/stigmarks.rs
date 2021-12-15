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
use serde::{Deserialize, Serialize};
use stigmarks_sql_rs::sql::collections::NaiveDateTime;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

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

// OPTIONS https://stigmark.stigmee.com/api/v1/stigmarks
#[options("/stigmarks", rank = 1)]
fn stigmarks_options() -> ServerResponse {
    ServerResponse::ok()
}

// POST https://stigmark.stigmee.com/api/v1/stigmarks
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

#[derive(Serialize)]
struct StigmarkResponsePublic {
    collection_id: u32,
    user_id: u32,
    user_name: String,
    urls: Vec<String>,
    keywords: Vec<String>,
    creation_date: NaiveDateTime,
}

// GET https://stigmark.stigmee.com/api/v1/stigmarks
#[get("/stigmarks?<from>", rank = 1)]
fn stigmarks_get(
    auth: JwtAuth,
    state: State<SqlStigmarksDB>,
    from: Option<u32>,
) -> ServerResponse {
    let mut user_id = 0u32;
    if let Some(claims) = auth.claims {
        user_id = claims.uid;
    }
    if user_id == 0 {
        println!("access denied");
        return ServerResponse::error("expected token", Status::Forbidden);
    }
    let mut stigmer_id = 0;
    if let Some(from_user_id) = from {
        stigmer_id = from_user_id;
    }
    let stigmarks_db = state.inner();
    let user = stigmarks_db.get_user_by_id(user_id);
    if let Err(err) = user {
        println!("could not find user: {}", err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    let user = user.unwrap();
    if let Some(disabled_at) = user.disabled_at {
        println!("user {} disabled at {}", user_id, disabled_at);
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    // todo: check we are follower of this user
    println!("get_all_collection: user_id={} stigmer_id={}", user_id, stigmer_id);
    let res = stigmarks_db.get_all_collections_from_user(user_id, stigmer_id);
    if let Err(err) = res {
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let collections = res
        .unwrap()
        .iter()
        .map(|c| {
            let user_id = c.user_id;
            let user_name = c.user_name.clone();
            let collection_id = c.id;
            let creation_date = c.creation_date;
            let urls = stigmarks_db.get_collection_urls_by_id(collection_id).unwrap();
            let keywords = stigmarks_db.get_collection_keywords_by_id(collection_id).unwrap();

            StigmarkResponsePublic {
                user_id,
                user_name,
                collection_id,
                creation_date,
                urls,
                keywords,
            }
        })
        .collect::<Vec<_>>();
    ServerResponse::json(json!(collections), Status::Ok)
}

pub fn routes() -> Vec<Route> {
    routes![stigmarks_options, stigmarks_post, stigmarks_get]
}
