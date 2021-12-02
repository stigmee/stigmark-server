// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : stigmarks-app-sql
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

use stigmarks::sql::SqlStigmarksDB;

const DB_USER: &str = "stigmark";
const DB_PASS: &str = "yAfisEra";

fn main() {
    let mut stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);

    println!("add_user");
    let user_id_0 = stigmarks_db.add_user(String::from("Philippe Anel"), String::from("zexigh@gmail.com"), vec!());
    match user_id_0 {
        Ok(user) => println!("\t{:?}", user),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_all_users");
    let all_users = stigmarks_db.get_all_users();
    match all_users {
        Ok(users) => println!("\t{:?}", users),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_user_by_id");
    let user_1 = stigmarks_db.get_user_by_id(1);
    match user_1 {
        Ok(user) => println!("\t{:?}", user),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_all_collections");
    let all_collections = stigmarks_db.get_all_collections();
    match all_collections {
        Ok(collections) => println!("\t{:?}", collections),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_collection_by_id");
    let collection_1 = stigmarks_db.get_collection_by_id(1);
    match collection_1 {
        Ok(collection) => println!("{:?}", collection),
        Err(err) => eprintln!("get_collection_by_id failed: {}", err),
    }
}
