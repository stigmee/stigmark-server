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
