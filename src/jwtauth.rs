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
use crate::config::COOKIE_NAME;

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
        let claims = claims.unwrap();
        println!("jwt-auth: claims={:?}", claims);
        Some(Self {
            claims: Some(claims),
        })
    }
}

#[derive(Debug)]
pub enum LoginError {
    InvalidToken,
    // NotImplemented,
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
        println!("from_request");

        let mut token = "";

        // can be provided through cookie
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get(COOKIE_NAME) {
            token = cookie.value();
            println!("got token by cookie: {}", token);
        }

        // can be provided through header "Authorization: Bearer xxxx"
        let keys: Vec<&str> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => { /* fall through */ }
            1 => {
                let key = keys[0];
                if key.len() > 7 && &key[..7] == "Bearer " {
                    token = &key[7..];
                    println!("got token by header: {}", token);
                }
            }
            _ => {
                println!("too many headers found");
                return Outcome::Failure((Status::BadRequest, LoginError::BadCount));
            }
        }

        if token == "" {
            println!("no token found");
            return Outcome::Success(JwtAuth { claims: None });
        }

        // can be provided through query : https://stigmark.stigmee.fr/?token=xxxx
        let token_by_query = value(request, "token").as_str();
        if token_by_query != "" {
            token = token_by_query;
            println!("got token by query: {}", token);
        }

        if let Some(auth_info) = JwtAuth::new(token) {
            println!("token is valid");
            return Outcome::Success(auth_info);
        }

        println!("could not get token_info");
        return Outcome::Failure((Status::BadRequest, LoginError::InvalidToken));
    }
}

use rocket::State;
use stigmarks_sql_rs::sql::SqlStigmarksDB;
use stigmarks_sql_rs::sql::users::SqlUser;

pub fn get_current_user(
    claims: &Claims,
    db_state: &State<SqlStigmarksDB>,
) -> Option<SqlUser> {
    let user_id = claims.uid;
    let stigmarks_db = db_state.inner();
    let user = stigmarks_db.get_user_by_id(user_id);
    if let Err(err) = user {
        println!("could not find user: {}", err);
        return None;
    }
    let user = user.unwrap();
    if let Some(_) = user.disabled_at {
        return None;
    }
    Some(user)
}
