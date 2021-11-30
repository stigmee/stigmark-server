use mysql::prelude::{Queryable};
use mysql::{params};
use mysql::chrono::NaiveDateTime;

pub use crate::sql::SqlStigmarksDB;

/*
CREATE TABLE IF NOT EXISTS `users` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(256) NOT NULL,
    `email` varchar(256) NOT NULL UNIQUE,
    `hash` binary(255) NOT NULL,
    `creation_date` datetime NOT NULL DEFAULT NOW(),
    PRIMARY KEY (`id`)
);
*/

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlUser {
    id: u32,
    name: String,
    email: String,
    // hidden: hash: Vec<u8>,
    creation_date: NaiveDateTime,
}

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn add_user(self: &mut Self, name: String, email: String, pass: Vec<u8>) -> Result<u32, String> {
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

    // todo: -> Result<SqlUser, Error>
    pub fn get_user_by_id(self: &mut Self, user_id: u32) -> Result<SqlUser, String> {
        match self.conn.exec_first(
            r"SELECT id, name, email, creation_date FROM users where id=:id",
            params! {
                "id" => user_id,
            },
        ) {
            Ok(row) => {
                match row.map(|(id, name, email, creation_date)| SqlUser {
                    id,
                    name,
                    email,
                    creation_date,
                }) {
                    Some(user) => Ok(user),
                    None => Err(format!("user {} not found", user_id)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }

    // todo: -> Result<Vec<SqlUser>, Error>
    pub fn get_all_users(self: &mut Self) -> Result<Vec<SqlUser>, String> {
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

    // todo: -> Result<SqlUser, Error>
    pub fn get_user_by_auth(self: &mut Self, user_email: &String, password_hash: Vec<u8>) -> Result<SqlUser, String> {
        match self.conn.exec_first(
            r"SELECT id, name, email, hash, creation_date FROM users where id=:id",
            params! {
                "email" => user_email,
                "hash" => password_hash,
            },
        ) {
            Ok(row) => {
                match row.map(|(id, name, email, creation_date)| SqlUser {
                    id,
                    name,
                    email,
                    creation_date,
                }) {
                    Some(user) => Ok(user),
                    None => Err(format!("user {} not found", user_email)),
                }
            },
            Err(err) => Err(format!("{}", err)),
        }
    }
}
