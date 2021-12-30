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
use rocket::http::{Status, Cookie, Cookies, SameSite};
use rocket_contrib::json::Json;
use rocket::{State, Route};
use rocket_contrib::json;
use crate::response::ServerResponse;
use crate::token::create_token;
use stigmarks_sql_rs::sql::SqlStigmarksDB;
use crate::config::COOKIE_NAME;

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

#[derive(Serialize)]
struct WhoAmIResult {
    pub id: u32,
    pub name: String,
    pub mail: String,
    pub is_private: bool,
    pub is_anonymous: bool,
}

#[options("/login")]
fn login_options() ->ServerResponse {
    ServerResponse::ok()
}

use crate::jwtauth::{JwtAuth, get_current_user};

// GET https://stigmark.stigmee.com/api/v1/login
#[get("/login", format = "json", rank = 1)]
fn login_get(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("stigmarks_get: who-am-i");

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }

    let current_user = current_user.unwrap();
    let whoami = WhoAmIResult {
        id: current_user.id,
        name: current_user.name,
        mail: current_user.email,
        is_private: current_user.is_private,
        is_anonymous: current_user.is_anonymous,
    };
    ServerResponse::json(json!(whoami), Status::Ok)
}

use std::str;

#[post("/login", format = "json", data = "<req>")]
fn login_post(state: State<SqlStigmarksDB>, mut cookies: Cookies, req: Json<LoginRequest>) -> ServerResponse {
    println!("login: user '{}' pass '****'", &req.mail);

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

    let cookie = Cookie::build(COOKIE_NAME, token.clone())
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .finish();
    cookies.add(cookie);

    let json = json!(LoginResult::new(token));
    ServerResponse::json(json, Status::Created)
}

#[delete("/login")]
fn login_delete(mut cookies: Cookies) {
    println!("logout");

    let cookie = Cookie::build(COOKIE_NAME, "")
        .path("/")
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .finish();
    cookies.remove(cookie);

    // ServerResponse::error("", Status::NoContent)
}

pub fn routes() -> Vec<Route> {
    routes![login_options, login_get, login_post, login_delete]
}
