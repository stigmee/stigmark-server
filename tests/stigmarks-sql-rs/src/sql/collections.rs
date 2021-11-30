use mysql::prelude::{Queryable};
use mysql::{params};

use crate::SqlCollection;
use crate::sql::SqlStigmarksDB;

#[allow(dead_code)]
impl SqlStigmarksDB {
    // todo: -> Result<SqlUser, Error>
    pub fn get_collection_by_id(self: &mut Self, collection_id: u32) -> Result<SqlCollection, String> {
        match self.conn.exec_first(
            r"SELECT id, user_id, creation_date, hidden FROM users where id=:id",
            params! {
                "id" => collection_id,
            },
        ) {
            Ok(row) => {
                match row.map(|(id, user_id, creation_date, hidden)| SqlUser {
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
            r"SELECT id, name, email, creation_date FROM users",
            {},
            |(id, name, email, creation_date)| SqlUser {
                id,
                name,
                email,
                creation_date,
            }
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_urls_by_id(self: &mut Self, collection_id: u32) -> Result<Vec<String>>, String> {
        match self.conn.exec_map(
            r"SELECT url FROM urls, url_lists where url_lists.collection_id=:collection_id and url.id=url_list.url_id",
            params! {
                "collection_id" => collection_id,
            },
            |(url)| String { url, },
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_collection_keywords_by_id(self: &mut Self, collection_id: u32) -> Result<Vec<String>>, String> {
        match self.conn.exec_map(
            r"SELECT keyword FROM keywords, keyword_lists where keyword_lists.collection_id=:collection_id and keyword.id=keyword_list.keyword_id",
            params! {
                "collection_id" => collection_id,
            },
            |(url)| String { url, },
        ) {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("{}", err)),
        }
    }
}
