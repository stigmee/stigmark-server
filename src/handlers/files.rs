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

use rocket::http::Status;
use rocket::Route;
use std::path::{Path, PathBuf};

use crate::basicauth::BasicAuth;
use crate::response::ServerResponse;
use rocket::State;

use crate::jwtauth::{get_current_user, JwtAuth};
use stigmarks_sql_rs::sql::SqlStigmarksDB;

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
fn files_slash(
    basic_auth: BasicAuth,
    jwt_auth: JwtAuth,
    file_state: State<FileState>,
    db_state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("stigmarks: GET /");

    let www_path = Path::new(&file_state.www_dir);
    let mut path = www_path.join("collections.htm");

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        let file_state = file_state.inner();
        if basic_auth.name != file_state.user || basic_auth.pass != file_state.pass {
            return ServerResponse::basic_auth();
        }

        path = www_path.join("login.htm");
    }

    println!("stigmarks: www-path={:?} file-path={:?}", www_path, path);
    ServerResponse::file(&path)
}

// GET https://stigmark.stigmee.com/*
#[get("/<file..>", rank = 3)]
fn files_others(
    basic_auth: BasicAuth,
    jwt_auth: JwtAuth,
    file_state: State<FileState>,
    db_state: State<SqlStigmarksDB>,
    file: PathBuf,
) -> ServerResponse {
    println!("files: GET {:?}", file);

    let www_path = Path::new(&file_state.www_dir);
    let mut path = www_path.join(file.clone());

    // forbid direct access
    let file_name = file.to_str().unwrap_or("");
    match file_name {
        "collections.htm" | "login.htm" | "" => {
            path = www_path.join("404.htm");
        }
        _ => {}
    }

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        let file_state = file_state.inner();
        if basic_auth.name != file_state.user || basic_auth.pass != file_state.pass {
            return ServerResponse::basic_auth();
        }
    }

    let ext = if let Some(ext) = path.extension() {
        ext.to_str().unwrap_or("")
    } else {
        ""
    };

    if ext == "htm" || ext == "html" {
        if let None = current_user {
            path = www_path.join("login.htm");
        } else if !path.exists() {
            path = www_path.join("404.htm");
        }
    } else if !path.exists() {
        return ServerResponse::error("not found", Status::NotFound);
    }

    println!("stigmarks: www-path={:?} file-path={:?}", www_path, path);
    ServerResponse::file(&path)
}

pub fn routes() -> Vec<Route> {
    routes![files_slash, files_others]
}
