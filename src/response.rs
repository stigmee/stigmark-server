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

use rocket::{Request, Response};
use rocket::response::{Responder, NamedFile};
use rocket::http::Status;
use rocket::response;
use rocket_contrib::json::JsonValue;
use std::path::Path;

#[derive(Debug)]
pub enum ServerResponse {
    Ok(),
    File(NamedFile),
    Json(JsonValue),
    Error(Status),
    BasicAuth(),
}

impl ServerResponse {
    pub fn ok() -> Self {
        Self::Ok()
    }

    pub fn file(file: &Path) -> Self {
        match NamedFile::open(file) {
            Ok(file) => Self::File(file),
            Err(_) => Self::Error(Status::NotFound),
        }
    }

    #[allow(dead_code)]
    pub fn json(json: JsonValue) -> Self {
        Self::Json(json)
    }

    pub fn basic_auth() -> Self {
        Self::BasicAuth()
    }
}

impl<'r>  Responder<'r> for ServerResponse {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        let mut res = Response::build();
        match self {
            Self::Ok() => res.status(Status::Ok),
            Self::File(file) => res.sized_body(file),
            Self::Error(status) => res.status(status),
            Self::BasicAuth() => {
                res
                    .raw_header("WWW-Authenticate", "Basic realm=\"User Visible Realm\", charset=\"UTF-8\"")
                    .status(Status::Unauthorized)
            }
            _ => res.status(Status::NotImplemented),
        }.ok()
    }
}
