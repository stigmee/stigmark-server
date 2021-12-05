// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : stigmarks-sql
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

use mysql::prelude::{Queryable};
use mysql::{params};
// use mysql::chrono::NaiveDateTime;

use crate::sql::SqlStigmarksDB;

/*
CREATE TABLE IF NOT EXISTS `collections` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `user_id` int(11) NOT NULL,
    `creation_date` datetime NOT NULL DEFAULT NOW(),
    `hidden` tinyint(1) NOT NULL DEFAULT FALSE,
    PRIMARY KEY (`id`),
    KEY `fk_collectionuser` (`user_id`),
    CONSTRAINT `fk_collections_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);
*/

#[derive(Debug, PartialEq, Eq)]
pub struct SqlCollection {
    pub id: u32,
    pub user_id: u32,
    pub creation_date: mysql::chrono::NaiveDateTime,
    pub hidden: bool,
}

/*
CREATE TABLE IF NOT EXISTS `keywords` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword` varchar(256) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `keyword` (`keyword`)
);

CREATE TABLE IF NOT EXISTS `keyword_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `keyword_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `keyword_id` (`keyword_id`),
    CONSTRAINT `fk_keyword_lists_collections_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_keyword_lists_keywords_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`id`)
);
*/

/*
CREATE TABLE IF NOT EXISTS `urls` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `url` varchar(2048) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `url` (`url`) USING HASH
);

CREATE TABLE IF NOT EXISTS `url_lists` (
    `collection_id` int(11) NOT NULL AUTO_INCREMENT,
    `url_id` int(11) NOT NULL,
    KEY `collection_id` (`collection_id`),
    KEY `url_id` (`url_id`),
    CONSTRAINT `fk_url_lists_collection_id` FOREIGN KEY (`collection_id`) REFERENCES `collections` (`id`),
    CONSTRAINT `fk_url_lists_url_id` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`)
);
*/

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlKeyword {
//     id: u32,
//     keyword: String,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlUrl {
//     id: u32,
//     url: String,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlKeywordList {
//     collection_id: u32,
//     keyword_id: u32,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlUrlList {
//     collection_id: u32,
//     url_id: u32,
// }

#[allow(dead_code)]
impl SqlStigmarksDB {
    fn get_keyword_id_by_name(self: &Self, keyword: &String) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id FROM keywords where keyword=:keyword",
            params! {
                "keyword" => keyword,
            },
        ) {
            Ok(row) => {
                match row.map(|id| id) {
                    Some(id) => Ok(id),
                    None => Err(format!("keyword {} not found", keyword)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn get_url_id_by_name(self: &Self, url: &String) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id FROM urls where url=:url",
            params! {
                "url" => url,
            },
        ) {
            Ok(row) => {
                match row.map(|id| id) {
                    Some(id) => Ok(id),
                    None => Err(format!("url {} not found", url)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn add_keyword(self: &Self, keyword: &String) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT INTO keywords (keyword) VALUES (:keyword) ON DUPLICATE KEY UPDATE ref_count = ref_count + 1",
            params! {
                    "keyword" => keyword,
            },
        ) {
            Ok(_) => {
                let mut keyword_id = conn.last_insert_id() as u32;
                if keyword_id == 0 {
                    keyword_id = match self.get_keyword_id_by_name(keyword) {
                        Ok(keyword_id) => keyword_id,
                        Err(_) => 0,
                    }
                };
                if keyword_id == 0 {
                    return Err(format!("could not find {} keyword_id", keyword));
                }
                Ok(keyword_id)
            }
            Err(err) => Err(format!("insert.err: {}", err)),
        }
    }

    fn add_url(self: &Self, url: &String) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT INTO urls (url) VALUES (:url) ON DUPLICATE KEY UPDATE ref_count = ref_count + 1",
            params! {
                    "url" => url,
            },
        ) {
            Ok(_) => {
                let mut url_id = conn.last_insert_id() as u32;
                if url_id == 0 {
                    url_id = match self.get_url_id_by_name(url) {
                        Ok(url_id) => url_id,
                        Err(_) => 0,
                    }
                };
                if url_id == 0 {
                    return Err(format!("could not find {} url_id", url));
                }
                Ok(url_id)
            }
            Err(err) => Err(format!("insert.err: {}", err)),
        }
    }

    fn add_keyword_to_collection(self: &Self, collection_id: u32, keyword_id: u32) -> Result<(), String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT IGNORE INTO keyword_lists (collection_id, keyword_id) VALUES (:collection_id, :keyword_id)",
            params! {
                    "collection_id" => collection_id,
                    "keyword_id" => keyword_id,
            },
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("insert.err: {}", err)),
        }
    }

    fn add_url_to_collection(self: &Self, collection_id: u32, url_id: u32) -> Result<(), String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT IGNORE INTO url_lists (collection_id, url_id) VALUES (:collection_id, :url_id)",
            params! {
                    "collection_id" => collection_id,
                    "url_id" => url_id,
            },
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("insert.err: {}", err)),
        }
    }

    // todo: -> Result<u32, Error>
    pub fn add_collection(self: &Self, user_id: u32, keywords: &Vec<String>, urls: &Vec<String>) -> Result<u32, String> {
        // create collection
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT INTO collections (user_id) VALUES (:user_id)",
            params! {
                    "user_id" => user_id,
            },
        ) {
            Ok(_) => {
                let collection_id = conn.last_insert_id() as u32;
                // add keywords
                for keyword in keywords {
                    match self.add_keyword(keyword) {
                        Ok(keyword_id) => {
                            match self.add_keyword_to_collection(collection_id, keyword_id) {
                                Ok(_) => {},
                                Err(err) => { return Err(err); },
                            }
                        },
                        Err(err) => {
                            return Err(err);
                        },
                    }
                }
                // add urls
                for url in urls {
                    match self.add_url(url) {
                        Ok(url_id) => {
                            match self.add_url_to_collection(collection_id, url_id) {
                                Ok(_) => {},
                                Err(err) => { return Err(err); },
                            }
                        },
                        Err(err) => {
                            return Err(err);
                        },
                    }
                }
                Ok(collection_id)
            }
            Err(err) => Err(format!("insert.collections.err: {}", err)),
        }
    }

    // todo: -> Result<SqlCollection, Error>
    pub fn get_collection_by_id(self: &Self, collection_id: u32) -> Result<SqlCollection, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id, user_id, creation_date, hidden FROM collections where id=:id",
            params! {
                "id" => collection_id,
            },
        ) {
            Ok(row) => {
                match row.map(|(id, user_id, creation_date, hidden)| SqlCollection {
                    id,
                    user_id,
                    creation_date,
                    hidden,
                }) {
                    Some(collection) => Ok(collection),
                    None => Err(format!("collection {} not found", collection_id)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    // todo: -> Result<Vec<SqlCollection>, Error>
    pub fn get_all_collections(self: &Self) -> Result<Vec<SqlCollection>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            r"SELECT id, user_id, creation_date, hidden FROM collections",
            {},
            |(id, user_id, creation_date, hidden)| SqlCollection {
                id,
                user_id,
                creation_date,
                hidden,
            }
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_urls_by_id(self: &Self, collection_id: u32) -> Result<Vec<String>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            r"SELECT url FROM urls, url_lists where url_lists.collection_id=:collection_id and urls.id=url_lists.url_id",
            params! {
                "collection_id" => collection_id,
            },
            |url| url,
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_keywords_by_id(self: &Self, collection_id: u32) -> Result<Vec<String>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            r"SELECT keyword FROM keywords, keyword_lists where keyword_lists.collection_id=:collection_id and keywords.id=keyword_lists.keyword_id",
            params! {
                "collection_id" => collection_id,
            },
            |keyword| keyword,
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }
}
