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
use rocket::State;

pub struct FileState {
    user: String,
    pass: String,
    www_dir: String,
}

impl FileState {
    pub fn new<S: Into<String>>(user: S, pass: S, dir: S) -> Self {
        FileState {
            user: user.into(),
            pass: pass.into(),
            www_dir: dir.into(),
        }
    }
}

// GET https://stigmark.stigmee.com/
#[get("/", rank = 2)]
fn files_slash(auth: BasicAuth, state: State<FileState>) -> ServerResponse {
    println!("stigmarks: '{}' GET /", auth.name);
    let state = state.inner();
    if auth.name != state.user || auth.pass != state.pass {
        return ServerResponse::basic_auth()
    }
    let www_path = Path::new(&state.www_dir);
    let path = www_path.join("index.htm");
    println!("stigmarks: www-path={:?} file-path={:?}", www_path, path);
    ServerResponse::file(&path)
}

// GET https://stigmark.stigmee.com/*
#[get("/<file..>", rank = 3)]
fn files_others(auth: BasicAuth, state: State<FileState>, file: PathBuf) -> ServerResponse {
    println!("stigmarks: '{}' GET {:?}", auth.name, file);
    let state = state.inner();
    if auth.name != state.user || auth.pass != state.pass {
        return ServerResponse::basic_auth()
    }
    let www_path = Path::new(&state.www_dir);
    let path = www_path.join(file);
    println!("stigmarks: www-path={:?} file-path={:?}", www_path, path);
    ServerResponse::file(&path)
}

pub fn routes() -> Vec<Route> {
    routes![files_slash, files_others]
}
