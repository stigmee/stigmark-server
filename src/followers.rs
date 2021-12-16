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

use crate::jwtauth::JwtAuth;
use crate::response::ServerResponse;
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json;

// OPTIONS https://stigmark.stigmee.com/api/v1/followers
#[options("/followers", rank = 1)]
fn followers_options() -> ServerResponse {
    ServerResponse::ok()
}

use stigmarks_sql_rs::sql::SqlStigmarksDB;

// GET https://stigmark.stigmee.com/api/v1/followers
#[get("/followers", rank = 1)]
fn followers_get(
    auth: JwtAuth,
    state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("followers_get");
    let mut user_id = 0u32;
    if let Some(claims) = auth.claims {
        user_id = claims.uid;
    }
    if user_id == 0 {
        println!("access denied");
        return ServerResponse::error("expected token", Status::Forbidden);
    }
    let stigmarks_db = state.inner();
    let user = stigmarks_db.get_user_by_id(user_id);
    if let Err(err) = user {
        println!("could not find user: {}", err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    let user = user.unwrap();
    if let Some(disabled_at) = user.disabled_at {
        println!("user {} disabled at {}", user_id, disabled_at);
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    // todo: check we are follower of this user
    println!("get_user_followers: user_id={}", user_id);
    let res = stigmarks_db.get_user_followers(user_id);
    if let Err(err) = res {
        eprintln!("get_user_followers failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let subscriptions = res.unwrap();
    ServerResponse::json(json!(subscriptions), Status::Ok)
}

pub fn routes() -> Vec<Route> {
    routes![followers_options, followers_get]
}
