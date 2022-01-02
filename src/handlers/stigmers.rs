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

use crate::jwtauth::{JwtAuth, get_current_user};
use crate::response::ServerResponse;
use rocket::http::Status;
use rocket::{Route, State};
use rocket_contrib::json;
use rocket_contrib::json::Json;
use serde::Deserialize;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

// OPTIONS https://stigmark.stigmee.com/api/v1/stigmers
#[options("/stigmers", rank = 1)]
fn stigmers_options() -> ServerResponse {
    ServerResponse::ok()
}

#[derive(Deserialize)]
struct AddStigmerRequest {
    user_mail: String,
}

// POST https://stigmark.stigmee.com/api/v1/stigmers
#[post("/stigmers", format = "json", data = "<req>", rank = 1)]
fn stigmers_post(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    req: Json<AddStigmerRequest>,
) -> ServerResponse {
    println!("stigmers_post: add subscription");

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
    let follower = stigmarks_db.get_user_by_id(current_user_id);
    if let Err(err) = follower {
        println!("could not find user {}: {}", current_user_id, err);
        return ServerResponse::error("user not found", Status::Forbidden);
    }

    let stigmer = stigmarks_db.get_user_by_email(&req.user_mail);
    if let Err(err) = stigmer {
        println!("could not find user {}: {}", req.user_mail, err);
        // todo: for security reason, we might have to "simulate" the existence of the requested user ... but this would require a "dummy user" ?
        return ServerResponse::error("user not found", Status::NotFound);
    }
    let stigmer = stigmer.unwrap();

    let stigmer_id = stigmer.id;
    let do_authorize = !stigmer.is_private; // if the account is not private, you can authorize immediatly
    let res = stigmarks_db.add_user_subscription(stigmer_id, current_user_id, do_authorize);
    if let Err(err) = res {
        eprintln!("add collection failed with: {}", err);
        if err.contains("Duplicate") {
            return ServerResponse::error(err, Status::Conflict);
        }
        return ServerResponse::error(err, Status::InternalServerError);
    }

    ServerResponse::json(json!({"collection_id": res.unwrap()}), Status::Created)
}

// GET https://stigmark.stigmee.com/api/v1/stigmers
#[get("/stigmers", rank = 1)]
fn stigmers_get(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
) -> ServerResponse {
    println!("stigmers_get: get follower subscription");

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
    let res = stigmarks_db.get_user_subscriptions(current_user_id);
    if let Err(err) = res {
        eprintln!("get_user_subscriptions failed with: {}", err);
        return ServerResponse::error(err, Status::InternalServerError);
    }

    let subscriptions = res.unwrap();
    ServerResponse::json(json!(subscriptions), Status::Ok)
}

// DELETE https://stigmark.stigmee.com/api/v1/followers/:follower_id
#[delete("/stigmers/<stigmer_id>", rank = 1)]
fn stigmers_delete(
    jwt_auth: JwtAuth,
    db_state: State<SqlStigmarksDB>,
    stigmer_id: u32,
) -> ServerResponse {
    println!("stigmers_delete: remove follower subscription");

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
    let res = stigmarks_db.remove_subscription(current_user_id, stigmer_id);
    if let Err(err) = res {
        // todo: detailed errors
        return ServerResponse::error(err, Status::BadRequest);
    }

    ServerResponse::error("", Status::NoContent)
}

pub fn routes() -> Vec<Route> {
    routes![stigmers_options, stigmers_post, stigmers_get, stigmers_delete]
}
