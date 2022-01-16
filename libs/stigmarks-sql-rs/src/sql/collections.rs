//
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021-2022 Philippe Anel <zexigh@gmail.com>
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

use crate::sql::SqlStigmarksDB;
pub use mysql::chrono::NaiveDateTime;
use mysql::params;
use mysql::prelude::Queryable;
use serde::Serialize;

pub type Error = String;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SqlCollection {
    pub id: u32,
    pub created_by: u32,
    pub created_at: NaiveDateTime,
    pub hidden_by: Option<NaiveDateTime>,
    pub hidden_at: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SqlCollectionPublic {
    pub id: u32,
    pub created_by: u32,
    pub user_name: String,
    pub created_at: NaiveDateTime,
}

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

#[derive(Serialize)]
pub struct SqlCollectionByKeywordUrlEntry {
    pub id: u32,
    pub url: String,
}

#[derive(Serialize)]
pub struct SqlCollectionByKeywordResponse {
    pub collection_id: u32,
    pub keyword_id: u32,
    pub urls: Vec<SqlCollectionByKeywordUrlEntry>,
}

#[allow(dead_code)]
impl SqlStigmarksDB {
    fn get_keyword_id_by_name(self: &Self, keyword: &String) -> Result<u32, Error> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id FROM keywords where keyword=:keyword",
            params! {
                "keyword" => keyword,
            },
        ) {
            Ok(row) => match row.map(|id| id) {
                Some(id) => Ok(id),
                None => Err(format!("keyword {} not found", keyword)),
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn get_url_id_by_name(self: &Self, url: &String) -> Result<u32, Error> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id FROM urls where url=:url",
            params! {
                "url" => url,
            },
        ) {
            Ok(row) => match row.map(|id| id) {
                Some(id) => Ok(id),
                None => Err(format!("url {} not found", url)),
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn add_keyword(self: &Self, keyword: &String) -> Result<u32, Error> {
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

    fn add_url(self: &Self, url: &String) -> Result<u32, Error> {
        let lock = self.url_mutex.lock();
        if let Err(poison) = lock {
            return Err(format!("mutex poison: {}", poison));
        }

        let conn = &mut self.pool.get_conn().expect("sql: could not connect");

        match conn.exec_first(
            r"SELECT id FROM urls WHERE urls.url=:url",
            params! {
                "url" => url,
            },
        ) {
            Ok(row) => match row {
                Some(url_id) => {
                    let res = conn.exec_drop(
                        r"UPDATE urls SET ref_count = ref_count + 1 WHERE id=:url_id",
                        params! {
                            "url_id" => url_id,
                        }
                    );
                    if let Err(err) = res {
                        return Err(format!("failed to update count: {}", err));
                    }
                    Ok(url_id)
                },
                None => {
                    match conn.exec_drop(
                        r"INSERT INTO urls (url) VALUES (:url)",
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
            },
            Err(err) => Err(format!("select.err: {}", err)),
        }
    }

    fn add_keyword_to_collection(
        self: &Self,
        collection_id: u32,
        keyword_id: u32,
    ) -> Result<(), Error> {
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

    fn add_url_to_collection(self: &Self, collection_id: u32, url_id: u32) -> Result<(), Error> {
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
    pub fn add_collection(
        self: &Self,
        created_by: u32,
        keywords: &Vec<String>,
        urls: &Vec<String>,
    ) -> Result<u32, Error> {
        // create collection
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_drop(
            r"INSERT INTO collections (created_by) VALUES (:created_by)",
            params! {
                    "created_by" => created_by,
            },
        ) {
            Ok(_) => {
                let collection_id = conn.last_insert_id() as u32;
                println!("added collection {}", collection_id);
                // add keywords
                println!("keywords count {}", keywords.len());
                for keyword in keywords {
                    if keyword.trim() == "" {
                        continue;
                    }
                    println!("adding keyword {}", keyword);
                    match self.add_keyword(keyword) {
                        Ok(keyword_id) => {
                            match self.add_keyword_to_collection(collection_id, keyword_id) {
                                Ok(_) => {}
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                // add urls
                println!("urls count {}", urls.len());
                for url in urls {
                    if url.trim() == "" {
                        continue;
                    }
                    println!("adding url {}", url);
                    match self.add_url(url) {
                        Ok(url_id) => match self.add_url_to_collection(collection_id, url_id) {
                            Ok(_) => {}
                            Err(err) => {
                                return Err(err);
                            }
                        },
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(collection_id)
            }
            Err(err) => Err(format!("insert.collections.err: {}", err)),
        }
    }

    // todo: -> Result<SqlCollection, Error>
    pub fn get_collection_by_id(self: &Self, collection_id: u32) -> Result<SqlCollection, Error> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_first(
            r"SELECT id, created_by, created_at, hidden_at, hidden_by FROM collections where id=:id",
            params! {
                "id" => collection_id,
            },
        ) {
            Ok(row) => {
                match row.map(|(id, created_by, created_at, hidden_at, hidden_by)| SqlCollection {
                    id,
                    created_by,
                    created_at,
                    hidden_at,
                        hidden_by,
                }) {
                    Some(collection) => Ok(collection),
                    None => Err(format!("collection {} not found", collection_id)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    // todo: -> Result<Vec<SqlCollection>, Error>
    pub fn get_all_collections(self: &Self) -> Result<Vec<SqlCollection>, Error> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            r"SELECT id, created_by, created_at, hidden_at, hidden_by FROM collections",
            {},
            |(id, created_by, created_at, hidden_at, hidden_by)| SqlCollection {
                id,
                created_by,
                created_at,
                hidden_at,
                hidden_by,
            },
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    // todo: -> Result<Vec<SqlCollection>, Error>
    pub fn get_all_collections_from_user(
        self: &Self,
        created_by: u32,
        _stigmer_id: u32,
    ) -> Result<Vec<SqlCollectionPublic>, Error> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            "   SELECT      C.id,
                            C.created_by,
                            U.name,
                            C.created_at
                FROM        collections C,
                            users U
                LEFT JOIN   followers F
                        ON  F.follower_id = :created_by
                WHERE       (       C.created_by = :created_by
                                OR  (
                                    C.created_by = F.stigmer_id
                                AND F.authorized_at IS NOT NULL
                                AND F.forbidden_at IS NULL )
        
                            )
                        AND C.hidden_at IS NULL
                        AND U.id = C.created_by
            ",
            params! {
                "created_by" => created_by,
            },
            |(id, created_by, user_name, created_at)| SqlCollectionPublic {
                id,
                created_by,
                user_name,
                created_at,
            },
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_urls_by_id(
        self: &Self,
        collection_id: u32,
    ) -> Result<Vec<String>, Error> {
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

    pub fn get_collection_keywords_by_id(
        self: &Self,
        collection_id: u32,
    ) -> Result<Vec<String>, Error> {
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

    pub fn get_collections_and_urls_by_keyword(
        self: &Self,
        keyword: String,
    ) -> Result<Vec<SqlCollectionByKeywordResponse>, Error> {
        let mut response: Vec<SqlCollectionByKeywordResponse> = vec!();

        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        match conn.exec_map(
            r"SELECT id FROM keywords WHERE LOWER(keywords.keyword)=LOWER(:keyword)",
            params! {
                "keyword" => keyword,
            },
            |id: u32| id,
        ) {
            Ok(keyword_ids) => {
                for keyword_id in keyword_ids {
                    match conn.exec_map(
                        r"SELECT collection_id FROM keyword_lists WHERE keyword_lists.keyword_id=:keyword_id",
                        params! {
                            "keyword_id" => keyword_id,
                        },
                        |collection_id: u32| collection_id,
                    ) {
                        Ok(collection_ids) => {
                            for collection_id in collection_ids {
                                match conn.exec_map(
                                    r"SELECT U.id, U.url FROM url_lists L, urls U WHERE L.collection_id=:collection_id AND U.id=L.url_id",
                                    params! {
                                        "collection_id" => collection_id,
                                    },
                                    |(id, url)| SqlCollectionByKeywordUrlEntry{
                                        id,
                                        url,
                                    },
                                ) {
                                    Ok(urls) => {
                                        response.push(SqlCollectionByKeywordResponse{
                                            collection_id,
                                            keyword_id,
                                            urls,
                                        });
                                    },
                                    Err(err) => return Err(format!("get_collections_by_keyword failed [1]: {}", err)),
                                }
                            }            
                        },
                        Err(err) => return Err(format!("get_collections_by_keyword failed [2]: {}", err))
                    }            
                }
            },
            Err(err) => return Err(format!("get_collections_by_keyword failed [3]: {}", err)),
        }
        Ok(response)
    }
}
