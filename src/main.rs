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
use rocket::fairing::AdHoc;
use cors::CORS;

// stigmark stuff
mod stigmarks;
mod token;
mod login;
mod signin;
mod files;
mod response;
mod basicauth;
mod cors;
use stigmarks_sql_rs::sql::SqlStigmarksDB;

fn main() {
    let mut api_routes = stigmarks::routes();
    api_routes.append(&mut login::routes());
    api_routes.append(&mut signin::routes());

    rocket::ignite()
        .attach(CORS)
        .attach(AdHoc::on_attach("db_cred", |rocket| {
            let val = rocket.config().get_string("db_cred").unwrap_or("".to_string());
            let creds: Vec<&str> = val.split(';').collect( );
            if creds.len() != 2 {
                println!("invalid credantial. Expected user;pass");
                return Err(rocket);
            }
            let stigmarks_db = SqlStigmarksDB::new(creds[0], creds[1]);
            if let Err(err) = stigmarks_db.init() {
                println!("stigmarks_db.init failed: {}", err);
                return Err(rocket);
            }
            Ok(rocket.manage(stigmarks_db))
        }))
        .mount("/", files::routes())
        .mount("/api/v1", api_routes)
        .launch();
}
