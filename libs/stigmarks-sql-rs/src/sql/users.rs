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

use mysql::chrono::NaiveDateTime;
use mysql::params;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

pub use crate::sql::SqlStigmarksDB;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    None,
    User,
    Admin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub validated_at: Option<NaiveDateTime>,
    pub disabled_at: Option<mysql::chrono::NaiveDateTime>,
    pub disabled_by: Option<u32>,
    pub is_private: bool,
    pub is_anonymous: bool,
}

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn add_user<S: Into<String>>(
        self: &Self,
        name: S,
        email: S,
        role: Role,
        pass: Vec<u8>,
    ) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"INSERT INTO users (name, email, role, hash) VALUES (:name, :email, :role, :hash)",
            params! {
                    "name" => name.into(),
                    "email" => email.into(),
                    "role" => role as u32,
                    "hash" => pass,
            },
        );
        if let Err(err) = res {
            return Err(format!("insert.err: {}", err));
        }
        Ok(conn.last_insert_id() as u32)
    }

    // todo: -> Result<SqlUser, Error>
    pub fn get_user_by_id(self: &Self, user_id: u32) -> Result<SqlUser, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_first(
            r"SELECT id, name, email, hash, created_at, validated_at, disabled_at, disabled_by, is_private, is_anonymous FROM users where id=:id",
            params! {
                "id" => user_id,
            },
        );
        if let Err(err) = res {
            return Err(format!("get_user_by_id failed: {}", err));
        }
        let row = res.unwrap();
        let res = row.map(
            |(
                id,
                name,
                email,
                hash,
                created_at,
                validated_at,
                disabled_at,
                disabled_by,
                is_private,
                is_anonymous,
            )| SqlUser {
                id,
                name,
                email,
                hash,
                created_at,
                validated_at,
                disabled_at,
                disabled_by,
                is_private,
                is_anonymous,
            },
        );
        if let None = res {
            return Err(format!("user {} not found", user_id));
        }
        Ok(res.unwrap())
    }

    // todo: -> Result<Vec<SqlUser>, Error>
    pub fn get_all_users(self: &Self) -> Result<Vec<SqlUser>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_map(
            r"SELECT id, name, email, hash, created_at, validated_at, disabled_at, disabled_by, is_private, is_anonymous FROM users",
            {},
            |(id, name, email, hash, created_at, validated_at, disabled_at, disabled_by, is_private, is_anonymous)| SqlUser {
                id,
                name,
                email,
                hash,
                created_at,
                validated_at,
                disabled_at,
                disabled_by,
                is_private,
                is_anonymous,
            },
        );
        if let Err(err) = res {
            return Err(format!("get_all_users failed: {}", err));
        }
        Ok(res.unwrap())
    }

    // todo: -> Result<SqlUser, Error>
    pub fn get_user_by_email(self: &Self, user_email: &String) -> Result<SqlUser, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_first(
            r"SELECT id, name, email, hash, created_at, validated_at, disabled_at, disabled_by, is_private, is_anonymous FROM users where email=:email",
            params! {
                "email" => user_email
            },
        );
        if let Err(err) = res {
            return Err(format!("get_user_by_auth failed: {}", err));
        }
        let row = res.unwrap();
        let res = row.map(
            |(
                id,
                name,
                email,
                hash,
                created_at,
                validated_at,
                disabled_at,
                disabled_by,
                is_private,
                is_anonymous,
            )| SqlUser {
                id,
                name,
                email,
                hash,
                created_at,
                validated_at,
                disabled_at,
                disabled_by,
                is_private,
                is_anonymous,
            },
        );
        if let None = res {
            return Err(format!("user '{}' not found", user_email));
        }
        Ok(res.unwrap())
    }

    // todo: -> Result<(), Error>
    pub fn set_user_privacy(
        self: &Self,
        _user_id: u32,
        _is_private: bool,
    ) -> Result<(), String> {
        Ok(())
    }

    // todo: -> Result<(), Error>
    pub fn set_user_anonymity(
        self: &Self,
        _user_id: u32,
        _is_anonymous: bool,
    ) -> Result<(), String> {
        Ok(())
    }
}
