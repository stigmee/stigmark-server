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
use serde::{Serialize};

pub use crate::sql::SqlStigmarksDB;

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

    // todo: -> Result<u32, Error>
    pub fn add_user_subscription(
        self: &Self,
        stigmer_id: u32,
        follower_id: u32,
        authorize: bool,
    ) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let mut req =
            r"INSERT INTO followers (stigmer_id, follower_id) VALUES (:stigmer_id, :follower_id)";
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
    pub fn get_user_followers(
        self: &Self,
        stigmer_id: u32,
    ) -> Result<Vec<SqlSubscription>, String> {
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
            |(
                stigmer_id,
                stigmer_name,
                stigmer_mail,
                follower_id,
                follower_name,
                follower_mail,
                authorized_at,
                forbidden_at,
            )| SqlSubscription {
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

    // todo: -> Result<(), Error>
    pub fn remove_subscription(
        self: &Self,
        stigmer_id: u32,
        follower_id: u32,
    ) -> Result<(), String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"DELETE FROM followers WHERE stigmer_id=:stigmer_id AND follower_id=:follower_id",
            params! {
                "stigmer_id" => stigmer_id,
                "follower_id" => follower_id,
            }
        );
        if let Err(err) = res {
            return Err(format!("{}", err));
        }
        Ok(())
    }

    // todo: -> Result<(), Error>
    pub fn authorize_follower_access_by_ids(
        self: &Self,
        stigmer_id: u32,
        follower_id: u32,
    ) -> Result<(), String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"UPDATE followers SET authorized_at=NOW(), forbidden_at=NULL WHERE stigmer_id=:stigmer_id AND follower_id=:follower_id",
            params! {
                "stigmer_id" => stigmer_id,
                "follower_id" => follower_id,
            }
        );
        if let Err(err) = res {
            return Err(format!("{}", err));
        }
        Ok(())
    }

    // todo: -> Result<(), Error>
    pub fn forbid_follower_access_by_ids(
        self: &Self,
        stigmer_id: u32,
        follower_id: u32,
    ) -> Result<(), String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"UPDATE followers SET authorized_at=NULL, forbidden_at=NOW() WHERE stigmer_id=:stigmer_id AND follower_id=:follower_id",
            params! {
                "stigmer_id" => stigmer_id,
                "follower_id" => follower_id,
            }
        );
        if let Err(err) = res {
            return Err(format!("{}", err));
        }
        Ok(())
    }

    // todo: -> Result<Vec<SqlSubscription>, Error>
    pub fn get_user_subscriptions(
        self: &Self,
        follower_id: u32,
    ) -> Result<Vec<SqlSubscription>, String> {
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
            |(
                stigmer_id,
                stigmer_name,
                stigmer_mail,
                follower_id,
                follower_name,
                follower_mail,
                authorized_at,
                forbidden_at,
            )| SqlSubscription {
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
