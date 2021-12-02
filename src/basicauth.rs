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

use rocket::request::{FromRequest, Outcome};
use rocket::{Request};
use rocket::http::Status;
use base64;

pub struct BasicAuth {
    pub name: String,
    pub pass: String,
}

impl BasicAuth {
    /// Creates a new [BasicAuth] struct/request guard from a given plaintext
    /// http auth header or returns a [Option::None] if invalid
    pub fn new<T: Into<String>>(auth_header: T) -> Option<Self> {
        println!("BasicAuth::new");
        
        let key = auth_header.into();
        if key.len() < 7 || &key[..6] != "Basic " {
            return None;
        }

        let decoded = match base64::decode(&key[6..]) {
            Ok(bytes) => String::from_utf8(bytes).unwrap(),
            Err(err) => format!("failed to decode {}", err),
        };

        let (name, pass) = match decoded.split_once(":") {
            Some((name, pass)) => (name, pass),
            None => ("", ""),
        };

        println!("BasicAuth: {}/{}", name, pass);

        Some(Self {
            name: String::from(name),
            pass: String::from(pass),
        })
    }
}

#[derive(Debug)]
pub enum LoginError {
    InvalidUserPassCombination,
    BadCount,
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("BasicAuth::FromRequest keys={}", keys.join(","));
        match keys.len() {
            0 => Outcome::Success(BasicAuth{name: String::from(""), pass: String::from("")}),
            1 => match BasicAuth::new(keys[0]) {
                Some(auth_header) => Outcome::Success(auth_header),
                None => Outcome::Failure((Status::BadRequest, LoginError::InvalidUserPassCombination)),
            },
            _ => Outcome::Failure((Status::BadRequest, LoginError::BadCount)),
        }
    }
}
