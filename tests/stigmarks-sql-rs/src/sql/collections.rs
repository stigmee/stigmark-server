use mysql::prelude::{Queryable};
use mysql::{params};
use mysql::chrono::NaiveDateTime;

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
    id: u32,
    user_id: u32,
    creation_date: mysql::chrono::NaiveDateTime,
    hidden: bool,
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
    /*
    pub fn add_collection(self: &mut Self, name: String, urls: Vec<String>, keywords: Vec<String>) -> Result<u32, String> {
        match self.conn.exec_drop(
            r"INSERT INTO users (name, email, hash) VALUES (:name, :email, :hash)",
            params! {
                    "name" => name,
                    "email" => email,
                    "hash" => pass,
            },
        ) {
            Ok(_) => Ok(self.conn.last_insert_id() as u32),
            Err(err) => Err(format!("insert.err: {}", err)),
        }
    }
    */

    // todo: -> Result<SqlUser, Error>
    pub fn get_collection_by_id(self: &mut Self, collection_id: u32) -> Result<SqlCollection, String> {
        match self.conn.exec_first(
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

    // todo: -> Result<Vec<SqlUser>, Error>
    pub fn get_all_collections(self: &mut Self) -> Result<Vec<SqlCollection>, String> {
        match self.conn.exec_map(
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

    pub fn get_collection_urls_by_id(self: &mut Self, collection_id: u32) -> Result<Vec<String>, String> {
        match self.conn.exec_map(
            r"SELECT url FROM urls, url_lists where url_lists.collection_id=:collection_id and url.id=url_list.url_id",
            params! {
                "collection_id" => collection_id,
            },
            |url| url,
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_keywords_by_id(self: &mut Self, collection_id: u32) -> Result<Vec<String>, String> {
        match self.conn.exec_map(
            r"SELECT keyword FROM keywords, keyword_lists where keyword_lists.collection_id=:collection_id and keyword.id=keyword_list.keyword_id",
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
