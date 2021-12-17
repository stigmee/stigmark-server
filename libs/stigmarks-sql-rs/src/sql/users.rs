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

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SqlSubscription {
    pub stigmer_id: u32,
    pub stigmer_name: String,
    pub stigmer_mail: String,
    pub follower_id: u32,
    pub follower_name: String,
    pub follower_mail: String,
    pub authorized_at: Option<NaiveDateTime>,
    pub forbidden_at: Option<NaiveDateTime>,
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
        let res = row.map(|(id, name, email, hash, created_at, validated_at, disabled_at, disabled_by, is_private, is_anonymous)| SqlUser {
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
        });
        if let None = res {
            return Err(format!("user '{}' not found", user_email));
        }
        Ok(res.unwrap())
    }

    // todo: -> Result<u32, Error>
    pub fn add_user_subscription(self: &Self, stigmer_id: u32, follower_id: u32, authorize: bool) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let mut req = r"INSERT INTO followers (stigmer_id, follower_id) VALUES (:stigmer_id, :follower_id)";
        if authorize {
            req = r"INSERT INTO followers (stigmer_id, follower_id, authorized_at) VALUES (:stigmer_id, :follower_id, NOW())"
        }
        let res = conn.exec_drop(
            req,
            params! {
                "stigmer_id" => stigmer_id,
                "follower_id" => follower_id,
            },
        );
        if let Err(err) = res {
            return Err(format!("add_user_subscription failed: {}", err));
        }
        Ok(conn.last_insert_id() as u32)
    }

    // todo: -> Result<Vec<SqlSubscription>, Error>
    pub fn get_user_followers(self: &Self, stigmer_id: u32) -> Result<Vec<SqlSubscription>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_map(
            r"  SELECT      U1.id as stigmer_id,
                            U1.name as stigmer_name,
                            U1.email as stigmer_mail,
                            U2.id as follower_id,
                            U2.name as follower_name,
                            U2.email as follower_mail,
                            F.authorized_at,
                            F.forbidden_at
                FROM        followers F,
                            users U1,
                            users U2
                WHERE       F.stigmer_id = :stigmer_id
                        AND	 U1.id = :stigmer_id
                        AND	 U2.id = F.follower_id
                        AND	 U1.disabled_at IS NULL
                        AND	 U2.disabled_at IS NULL
            ",
            params! {
                "stigmer_id" => stigmer_id,
            },
            |(stigmer_id, stigmer_name, stigmer_mail, follower_id, follower_name, follower_mail, authorized_at, forbidden_at)| SqlSubscription {
                stigmer_id,
                stigmer_name,
                stigmer_mail,
                follower_id,
                follower_name,
                follower_mail,
                authorized_at,
                forbidden_at,
            },
        );
        if let Err(err) = res {
            return Err(format!("get_user_followers failed: {}", err));
        }
        Ok(res.unwrap())
    }

    // todo: -> Result<Vec<SqlSubscription>, Error>
    pub fn get_user_subscriptions(self: &Self, follower_id: u32) -> Result<Vec<SqlSubscription>, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_map(
            r"  SELECT      U1.id as stigmer_id,
                            U1.name as stigmer_name,
                            U1.email as stigmer_mail,
                            U2.id as follower_id,
                            U2.name as follower_name,
                            U2.email as follower_mail,
                            F.authorized_at,
                            F.forbidden_at
                FROM        followers F,
                            users U1,
                            users U2
                WHERE       F.follower_id = :follower_id
                        AND	 U2.id = :follower_id
                        AND	 U1.id = F.stigmer_id
                        AND	 U1.disabled_at IS NULL
                        AND	 U2.disabled_at IS NULL
            ",
            params! {
                "follower_id" => follower_id,
            },
            |(stigmer_id, stigmer_name, stigmer_mail, follower_id, follower_name, follower_mail, authorized_at, forbidden_at)| SqlSubscription {
                stigmer_id,
                stigmer_name,
                stigmer_mail,
                follower_id,
                follower_name,
                follower_mail,
                authorized_at,
                forbidden_at,
            },
        );
        if let Err(err) = res {
            return Err(format!("get_user_subscriptions failed: {}", err));
        }
        Ok(res.unwrap())
    }
}
