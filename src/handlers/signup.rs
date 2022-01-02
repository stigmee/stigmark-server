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

use serde::{Deserialize, Serialize};
use rocket::http::{Status, Cookie, Cookies, SameSite};
use rocket_contrib::json::Json;
use rocket::{State, Route};
use rocket_contrib::json;
use crate::response::ServerResponse;
use crate::token::{create_token};
use stigmarks_sql_rs::sql::{SqlStigmarksDB, users::Role};
use crate::config::COOKIE_NAME;

#[derive(Deserialize)]
struct SignupRequest {
    user: String,
    mail: String,
    pass: String,
}

#[derive(Serialize)]
struct SignupResult {
    token: String,
}

#[allow(dead_code)]
impl SignupResult {
    fn new<S: Into<String>>(token: S) -> Self {
        SignupResult {
            token: token.into(),
        }
    }
}

#[options("/signup")]
fn signup_options() ->ServerResponse {
    ServerResponse::ok()
}

#[post("/signup", format = "json", data = "<req>")]
fn signup_post(state: State<SqlStigmarksDB>, mut cookies: Cookies, req: Json<SignupRequest>) -> ServerResponse {
    println!("signup: user '{}' pass '{}'", &req.mail, &req.pass);
    let passwd = &req.pass;
    let hash = bcrypt::hash(passwd, 6).unwrap();
    let stigmarks_db = state.inner();
    let res = stigmarks_db.add_user(&req.user, &req.mail, Role::User, hash.as_str().as_bytes().to_vec());
    if let Err(err) = res { 	
        eprintln!("add user failed with: {}", err);
        if err.contains("Duplicate") {
            return ServerResponse::error(err, Status::Conflict);
        }
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let token = create_token(res.unwrap()).unwrap();

    let cookie = Cookie::build(COOKIE_NAME, token.clone())
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .finish();
    cookies.add_private(cookie);

    let json = json!(SignupResult::new(token));
    ServerResponse::json(json, Status::Created)
}

pub fn routes() -> Vec<Route> {
    routes![signup_options, signup_post]
}
