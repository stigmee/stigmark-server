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

use crate::jwtauth::{JwtAuth, get_current_user};
use crate::response::ServerResponse;
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

// OPTIONS https://stigmark.stigmee.com/api/v1/followers
#[options("/followers", rank = 1)]
fn followers_options() -> ServerResponse {
    ServerResponse::ok()
}

// GET https://stigmark.stigmee.com/api/v1/followers
#[get("/followers", rank = 1)]
fn followers_get(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("followers_get: enumerate followers");
 
    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }
    let current_user = current_user.unwrap();
    let current_user_id = current_user.id;
 
    let stigmarks_db = db_state.inner();
    let subscriptions = stigmarks_db.get_user_followers(current_user_id);
    if let Err(err) = subscriptions {
        eprintln!("get_user_followers failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
 
    let subscriptions = subscriptions.unwrap();
    ServerResponse::json(json!(subscriptions), Status::Ok)
}

// PUT https://stigmark.stigmee.com/api/v1/followers/:follower_id
#[put("/followers/<follower_id>", rank = 1)]
fn followers_put(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    follower_id: u32,
) -> ServerResponse {
    println!("followers_put: authorize follower subscription");

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }
    let current_user = current_user.unwrap();
    let current_user_id = current_user.id;
 
    let stigmarks_db = db_state.inner();
    let res = stigmarks_db.authorize_follower_access_by_ids(current_user_id, follower_id);
    if let Err(err) = res {
        // todo: detailed errors
        return ServerResponse::error(err, Status::BadRequest);
    }

    ServerResponse::error("", Status::NoContent)
}

// DELETE https://stigmark.stigmee.com/api/v1/followers/:follower_id
#[delete("/followers/<follower_id>", rank = 1)]
fn followers_delete(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    follower_id: u32,
) -> ServerResponse {
    println!("followers_delete: forbid follower subscription");

    let current_user = if let Some(claims) = jwt_auth.claims {
        get_current_user(&claims, &db_state)
    } else {
        None
    };
    if let None = current_user {
        return ServerResponse::error("not logged", Status::Forbidden);
    }
    let current_user = current_user.unwrap();
    let current_user_id = current_user.id;
 
    let stigmarks_db = db_state.inner();
    let res = stigmarks_db.forbid_follower_access_by_ids(current_user_id, follower_id);
    if let Err(err) = res {
        // todo: detailed errors
        return ServerResponse::error(err, Status::BadRequest);
    }

    ServerResponse::error("", Status::NoContent)
}

pub fn routes() -> Vec<Route> {
    routes![followers_options, followers_get, followers_put, followers_delete]
}
