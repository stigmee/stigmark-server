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

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// rocket stuff
use rocket::config::{Config, Environment};

// stigmark stuff
mod stigmarks;
mod login;
mod files;
mod response;
mod basicauth;
mod cors;

use stigmarks_sql_rs::sql::SqlStigmarksDB;
// use std::sync::Mutex;
use cors::CORS;

const DB_USER: &str = "stigmark";
const DB_PASS: &str = "yAfisEra";

fn main() {
    let stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);

    // todo: remove this. We need it to create user 1
    if let Ok(user_id) = stigmarks_db.add_user("Philippe Anel", "zexigh@gmail.com", vec![]) {
        println!("user {} added", user_id);
    }

    // start the web service
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    let mut api_routes = stigmarks::routes();
    api_routes.append(&mut login::routes());

    rocket::custom(config)
        .manage(stigmarks_db)
        .attach(CORS)
        .mount("/", files::routes())
        .mount("/api/v1", api_routes)
        .launch();
}
