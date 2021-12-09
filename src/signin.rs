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

use serde::{Deserialize, Serialize};
use rocket::http::{Status};
use rocket_contrib::json::Json;
use std::fmt::Write;
use rocket::{State, Route};
use rocket_contrib::json;
use crate::response::ServerResponse;
use crate::token::create_token;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

#[derive(Deserialize)]
struct SigninRequest {
    user: String,
    mail: String,
    pass: String,
}

#[derive(Serialize)]
struct SigninResult {
    token: String,
}

#[allow(dead_code)]
impl SigninResult {
    fn new<S: Into<String>>(token: S) -> Self {
        SigninResult {
            token: token.into(),
        }
    }
}

#[options("/signin")]
fn signin_options() ->ServerResponse {
    ServerResponse::ok()
}

#[post("/signin", format = "json", data = "<req>")]
fn signin_post(state: State<SqlStigmarksDB>, req: Json<SigninRequest>) -> ServerResponse {
    let passwd = &req.pass;
    let hash = bcrypt::hash(passwd, 6).unwrap();
    let stigmarks_db = state.inner();
    let res = stigmarks_db.add_user(req.user, req.mail, hash.as_bytes().iter().collect());
    if let Err(err) = res { 	
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let token = create_token(1).unwrap();
    let json = json!(SigninResult::new(token));
    ServerResponse::json(json, Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![signin_options, signin_post]
}
