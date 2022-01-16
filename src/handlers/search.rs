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

// use crate::jwtauth::JwtAuth;
use crate::response::ServerResponse;
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json;
// use rocket_contrib::json::Json;
// use serde::Deserialize;

// OPTIONS https://stigmark.stigmee.com/api/v1/search
#[options("/search", rank = 1)]
fn search_options() -> ServerResponse {
    ServerResponse::ok()
}

use stigmarks_sql_rs::sql::SqlStigmarksDB;

// GET https://stigmark.stigmee.com/api/v1/search
#[get("/search?<q>", rank = 1)]
fn search_get(
    state: State<SqlStigmarksDB>,
    q: Option<String>,
) -> ServerResponse {
    let stigmarks_db = state.inner();
    if let None = q {
        return ServerResponse::error(format!("no keyword"), Status::BadRequest);
    }

    let response = stigmarks_db.get_collections_and_urls_by_keyword(q.unwrap());
    if let Err(err) = response {
        return ServerResponse::error(err, Status::InternalServerError);
    }

    let response = response.unwrap();
    ServerResponse::json(json!(response), Status::Ok)
}

pub fn routes() -> Vec<Route> {
    routes![search_options, search_get]
}
