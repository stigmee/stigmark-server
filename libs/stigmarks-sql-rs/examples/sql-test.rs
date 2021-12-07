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

use stigmarks_sql_rs::sql::SqlStigmarksDB;

const DB_USER: &str = "stigmark";
const DB_PASS: &str = "yAfisEra";

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::thread;

fn main() {
    let stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);

    if let Err(err) = stigmarks_db.init() {
        eprintln!("\tinit failed: {}", err);
    }

    println!("add_user");
    let user_id_0 = stigmarks_db.add_user(
        String::from("Philippe Anel"),
        String::from("zexigh@gmail.com"),
        vec![],
    );
    match user_id_0 {
        Ok(user) => println!("\t{:?}", user),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_all_users");
    let all_users = stigmarks_db.get_all_users();
    match all_users {
        Ok(users) => println!("\tgot {} users", users.len()),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_user_by_id");
    let user_1 = stigmarks_db.get_user_by_id(1);
    match user_1 {
        Ok(user) => println!("\t{:?}", user),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("add_collecton 1");
    let coll_1 = stigmarks_db.add_collection(
        1,
        &vec!["foo".to_string(), "bar".to_string()],
        &vec!["https//google.fr".to_string()],
    );
    match coll_1 {
        Ok(collection_id) => println!("\t{:?}", collection_id),
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    let mut handles = vec!();
    for t in 0..5 {
        let stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);
        handles.push(thread::spawn(move || {
            let rng = &mut thread_rng();

            let nc = 5; // rng.gen::<u32>() % 200;
            for n in 1..nc {
                let w0 = rng.gen::<usize>() % 30;
                let rand_string: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(w0)
                    .map(char::from)
                    .collect();

                let req = stigmarks_db.add_user(
                    format!("user {}", rand_string),
                    format!("zexigh@{}.com", rand_string),
                    vec![],
                );
                let user_id = if let Ok(user_id) = req {
                    user_id
                } else {
                    1
                };

                let mut urls = vec!["https://google.com".to_string()];
                let u = rng.gen::<u32>() % 5 + 1;
                for _ in 1..u {
                    let w1 = rng.gen::<usize>() % 30;
                    let rand_string: String = rng
                        .sample_iter(&Alphanumeric)
                        .take(w1)
                        .map(char::from)
                        .collect();
                    urls.push(format!("https://{}.com", rand_string))
                }

                let mut keywords = vec!["foo".to_string()];
                let k = rng.gen::<u32>() % 2 + 1;
                for _ in 1..k {
                    let w2 = rng.gen::<usize>() % 30;
                    let rand_string: String = rng
                        .sample_iter(&Alphanumeric)
                        .take(w2)
                        .map(char::from)
                        .collect();
                    keywords.push(format!("{}", rand_string))
                }

                let coll_1 = stigmarks_db.add_collection(user_id, &keywords, &urls);
                match coll_1 {
                    Ok(collection_id) => if n % 300 == 0 {
                        println!("{}: add_collection {}/{} - {} u={} k={}", t, n, nc, collection_id, u, k);
                    },
                    Err(err) => eprintln!("\tfailed: {}", err),
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("get_all_collections");
    let all_collections = stigmarks_db.get_all_collections();
    match all_collections {
        Ok(collections) => {
            let mut n = 0;
            let mut u = 0;
            let mut k = 0;
            for collection in &collections {
                n += 1;
                if n < 10 {
                    println!("\t{:?}", collection);
                }
                let collection_keywords = stigmarks_db.get_collection_keywords_by_id(collection.id);
                match collection_keywords {
                    Ok(keywords) => {
                        k += keywords.len();
                        if n < 10 {
                            println!("\t\t{} keywords", keywords.len());
                        }
                    }
                    Err(err) => eprintln!("\t\tfailed: {}", err),
                }
                let collection_urls = stigmarks_db.get_collection_urls_by_id(collection.id);
                match collection_urls {
                    Ok(urls) => {
                        u += urls.len();
                        if n < 10 {
                            println!("\t\t{} urls", urls.len());
                        }
                    }
                    Err(err) => eprintln!("\t\tfailed: {}", err),
                }
            }
            println!("{} entries, {} keywords, {} urls", n, k, u);
        }
        Err(err) => eprintln!("\tfailed: {}", err),
    }

    println!("get_collection_by_id");
    let collection_1 = stigmarks_db.get_collection_by_id(1);
    match collection_1 {
        Ok(collection) => println!("\t{:?}", collection),
        Err(err) => eprintln!("\tfailed: {}", err),
    }
}
