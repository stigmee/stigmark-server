//
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021-2022 Philippe Anel <zexigh@gmail.com>
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

use crate::jwtauth::{JwtAuth, get_current_user};
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
}

// OPTIONS https://stigmark.stigmee.com/api/v1/stigmarks
#[options("/stigmarks", rank = 1)]
fn stigmarks_options() -> ServerResponse {
    ServerResponse::ok()
}

// POST https://stigmark.stigmee.com/api/v1/stigmarks
#[post("/stigmarks", format = "json", data = "<add_collection_request>", rank = 1)]
fn stigmarks_post(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    add_collection_request: Json<StigmarkRequest>,
) -> ServerResponse {
    println!("stigmarks_post: add_collection");

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }
    let current_user = current_user.unwrap();
    let current_user_id = current_user.id;
 
    let stigmarks_db = db_state.inner();
    if let Err(err) = stigmarks_db.get_user_by_id(current_user_id) {
        println!("could not find user: {}", err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }

    let res = stigmarks_db.add_collection(
        current_user_id, &add_collection_request.keys, &add_collection_request.urls);
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
#[get("/stigmarks?<optional_user_id>", rank = 1)]
fn stigmarks_get(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    optional_user_id: Option<u32>,
) -> ServerResponse {
    println!("stigmarks_get: get_all_collection");

    let mut stigmer_id = 0;
    if let Some(from_user_id) = optional_user_id {
        stigmer_id = from_user_id;
    }

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }
    let current_user = current_user.unwrap();
    let current_user_id = current_user.id;
 
    // todo: check we are follower of this user
    println!("get_all_collection: user_id={} stigmer_id={}", current_user_id, stigmer_id);

    let stigmarks_db = db_state.inner();
    let collections = stigmarks_db.get_all_collections_from_user(current_user_id, stigmer_id);
    if let Err(err) = collections {
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let collections = collections
        .unwrap()
        .iter()
        .map(|c| {
            let user_id = c.created_by;
            let user_name = c.user_name.clone();
            let collection_id = c.id;
            let creation_date = c.created_at;
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
