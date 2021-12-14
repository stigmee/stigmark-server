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

use std::path::{Path, PathBuf};
use rocket::Route;

use crate::response::ServerResponse;
use crate::basicauth::BasicAuth;

// GET https://stigmark.stigmee.com/
#[get("/", rank = 2)]
fn files_slash(auth: BasicAuth) -> ServerResponse {
    println!("stigmarks: '{}' GET /", auth.name);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/index.htm");
    ServerResponse::file(&path)
}

// GET https://stigmark.stigmee.com/*
#[get("/<file..>", rank = 3)]
fn files_others(auth: BasicAuth, file: PathBuf) -> ServerResponse {
    println!("stigmarks: '{}' GET {:?}", auth.name, file);
    if auth.name != "stigmer" || auth.pass != "tabarnak" {
        return ServerResponse::basic_auth()
    }
    let path = Path::new("www/").join(file);
    ServerResponse::file(&path)
}

pub fn routes() -> Vec<Route> {
    routes![files_slash, files_others]
}
