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
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::token::Claims;

#[allow(dead_code)]
pub struct JwtAuth {
    pub claims: Option<Claims>,
}

impl JwtAuth {
    /// Creates a new [JwtAuth] struct/request guard from a given plaintext
    /// http auth header or returns a [Option::None] if invalid
    pub fn new<T: Into<String>>(token: T) -> Option<Self> {
        let claims = Claims::decode_from(token.into());
        if let Err(err) = claims {
            eprintln!("Jwt error: {}", err);
            return None;
        }
        Some(Self { claims: Some(claims.unwrap()) })
    }
}

#[derive(Debug)]
pub enum LoginError {
    InvalidToken,
    NotImplemented,
    BadCount,
}

use rocket::http::RawStr;

fn value<'s>(req: &'s Request, key: &str) -> &'s RawStr {
    req.get_query_value(key)
        .and_then(|r| r.ok())
        .unwrap_or("".into())
}

impl<'a, 'r> FromRequest<'a, 'r> for JwtAuth {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // can be provided through cookie
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get("stigmark") {
            println!("cookie: {:?}", cookie);
            return Outcome::Failure((Status::BadRequest, LoginError::NotImplemented));
        }

        // can be provided through query : https://domain.ltd/?token=xxxx
        let token = value(request, "token").as_str();
        if token != "" {
            println!("token[query]: {:?}", token);
            return Outcome::Failure((Status::BadRequest, LoginError::NotImplemented));
        }

        // can be provided through header
        let keys: Vec<&str> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Success(JwtAuth { claims: None }),
            1 => {
                let key = keys[0];
                if key.len() > 7 && &key[..7] == "Bearer " {
                    if let Some(auth_info) = JwtAuth::new(&key[7..]) {
                        return Outcome::Success(auth_info);
                    }
                }
                Outcome::Failure((Status::BadRequest, LoginError::InvalidToken))
            }
            _ => Outcome::Failure((Status::BadRequest, LoginError::BadCount)),
        }
    }
}
