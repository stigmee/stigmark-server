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
use rocket::{State, Route};
use rocket_contrib::json;
use crate::response::ServerResponse;
use crate::token::create_token;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

#[derive(Deserialize)]
struct LoginRequest {
    mail: String,
    pass: String,
}

#[derive(Serialize)]
struct LoginResult {
    token: String,
}

#[allow(dead_code)]
impl LoginResult {
    fn new<S: Into<String>>(token: S) -> Self {
        LoginResult {
            token: token.into(),
        }
    }
}

#[options("/login")]
fn login_options() ->ServerResponse {
    ServerResponse::ok()
}

use std::str;

#[post("/login", format = "json", data = "<req>")]
fn login_post(state: State<SqlStigmarksDB>, req: Json<LoginRequest>) -> ServerResponse {
    println!("login: user '{}' pass '{}'", &req.mail, &req.pass);
    if req.mail == "" || req.pass == "" {
        eprintln!("login: invalid parameters");
        return ServerResponse::error("invalid parameters", Status::BadRequest);
    }
    let stigmarks_db = state.inner();
    let res = stigmarks_db.get_user_by_email(&req.mail);
    if let Err(err) = res { 	
        eprintln!("get user {} failed with: {}", &req.mail, err);
        return ServerResponse::error("user not found", Status::Unauthorized);
    }
    // todo: remove unwraps
    let user = res.unwrap();
    if !bcrypt::verify(&req.pass, str::from_utf8(&user.hash).unwrap()).unwrap() {
        eprintln!("get user '{}' failed: invalid user/pass combination", &req.mail);
        return ServerResponse::error("invalid user/pass combination", Status::Unauthorized);
    }
    let token = create_token(user.id).unwrap();
    let json = json!(LoginResult::new(token));
    ServerResponse::json(json, Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![login_options, login_post]
}
