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
use rocket_contrib::json::Json;
use serde::Deserialize;

// OPTIONS https://stigmark.stigmee.com/api/v1/stigmers
#[options("/stigmers", rank = 1)]
fn stigmers_options() -> ServerResponse {
    ServerResponse::ok()
}

use stigmarks_sql_rs::sql::SqlStigmarksDB;

#[derive(Deserialize)]
struct AddStigmerRequest {
    user_mail: String,
}

// POST https://stigmark.stigmee.com/api/v1/stigmers
#[post("/stigmers", format = "json", data = "<req>", rank = 1)]
fn stigmers_post(
    auth: JwtAuth,
    state: State<SqlStigmarksDB>,
    req: Json<AddStigmerRequest>,
) -> ServerResponse {
    let mut follower_id = 0u32;
    if let Some(claims) = auth.claims {
        follower_id = claims.uid;
    }
    if follower_id == 0 {
        println!("access denied");
        return ServerResponse::error("expected token", Status::Forbidden);
    }
    let stigmarks_db = state.inner();

    let follower = stigmarks_db.get_user_by_id(follower_id);
    if let Err(err) = follower {
        println!("could not find user {}: {}", follower_id, err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }

    let stigmer = stigmarks_db.get_user_by_email(&req.user_mail);
    if let Err(err) = stigmer {
        println!("could not find user {}: {}", req.user_mail, err);
        // todo: for security reason, we might "simulate" the existence of the requested user ... but this would require a "dummy user" ?
        return ServerResponse::error("user not found", Status::Forbidden);
    }
    let stigmer = stigmer.unwrap();

    let stigmer_id = stigmer.id;
    let do_authorize = !stigmer.is_private; // if the account is not private, you can authorize immediatly
    let res = stigmarks_db.add_user_subscription(stigmer_id, follower_id, do_authorize);
    if let Err(err) = res {
        eprintln!("add collection failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    ServerResponse::json(json!({"collection_id": res.unwrap()}), Status::Created)
}

// GET https://stigmark.stigmee.com/api/v1/stigmers
#[get("/stigmers", rank = 1)]
fn stigmers_get(
    auth: JwtAuth,
    state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("stigmers_get");
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
    println!("get_user_subscriptions: user_id={}", user_id);
    let res = stigmarks_db.get_user_subscriptions(user_id);
    if let Err(err) = res {
        eprintln!("get_user_subscriptions failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }
    let subscriptions = res.unwrap();
    ServerResponse::json(json!(subscriptions), Status::Ok)
}

pub fn routes() -> Vec<Route> {
    routes![stigmers_options, stigmers_post, stigmers_get]
}
