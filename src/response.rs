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
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::path::Path;

#[derive(Debug)]
pub enum ServerResponse {
    Ok(),
    File(NamedFile),
    Json(JsonValue, Status),
    Error(String, Status),
    BasicAuth(),
}

impl ServerResponse {
    pub fn file(file: &Path) -> Self {
        match NamedFile::open(file) {
            Ok(file) => Self::File(file),
            Err(_) => Self::Error("".to_string(), Status::NotFound),
        }
    }

    pub fn ok() -> Self {
        Self::Ok()
    }

    #[allow(dead_code)]
    pub fn json(json: JsonValue, status: Status) -> Self {
        Self::Json(json, status)
    }

    pub fn basic_auth() -> Self {
        Self::BasicAuth()
    }

    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html 
    pub fn error<S: Into<String>> (msg: S, status: Status) -> Self {
        Self::Error(msg.into(), status)
    }
}

impl<'r>  Responder<'r> for ServerResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut res = Response::build();
        match self {
            Self::Ok() => res.status(Status::Ok),
            Self::File(file) => {
                let mut ct = "application/octet-stream";
                if let Some(ext) = file.path().extension() {
                    if let Some(ext) = ext.to_str() {
                        // todo: there must be a library for this
                        match ext {
                            "html" | "htm" => {
                                ct = "text/html";
                            },
                            "js" => {
                                ct = "application/javascript";
                            },
                            "json" => {
                                ct = "application/json";
                            },
                            "woff" => {
                                ct = "font/woff";
                            },
                            "woff2" => {
                                ct = "font/woff2";
                            },
                            "css" => {
                                ct = "text/css";
                            },
                            _ => {},
                        }
                    }
                }
                res
                    .raw_header("Content-Type", ct)
                    .streamed_body(file)
            },
            Self::Json(json, status) => {
                res.merge(Response::build_from(json.respond_to(&req).unwrap()).finalize())
                    .header(ContentType::JSON)
                    .status(status)
            }
            Self::Error(msg, status) => {
                let json = json!({"msg": msg});
                res.merge(Response::build_from(json.respond_to(&req).unwrap()).finalize())
                    .header(ContentType::JSON)
                    .status(status)
            },
            Self::BasicAuth() => {
                res
                    .raw_header("WWW-Authenticate", "Basic realm=\"User Visible Realm\", charset=\"UTF-8\"")
                    .status(Status::Unauthorized)
            }
        }.ok()
    }
}
