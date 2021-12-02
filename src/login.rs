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
use rocket::Route;
use crate::response::ServerResponse;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    passwd: String,
}

#[derive(Serialize)]
struct LoginResult {
    token: String,
}

#[options("/login")]
fn login_options() ->ServerResponse {
    ServerResponse::ok()
}

#[post("/login", format = "json", data = "<req>")]
fn login_post(req: Json<LoginRequest>) -> Status {
    let passwd = &req.passwd;
    let hash = bcrypt::hash(passwd, 6).unwrap();
    let mut hash_string = String::new();
    for byte in hash.bytes() {
        write!(&mut hash_string, "{:X}", byte).expect("Unable to write");
    }
    println!("user: {} -> {}", req.email, hash_string);
    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![login_options, login_post]
}
