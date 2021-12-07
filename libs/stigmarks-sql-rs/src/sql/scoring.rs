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

use mysql::prelude::Queryable;
use mysql::params;
pub use crate::sql::SqlStigmarksDB;

#[derive(Debug, PartialEq)]
pub struct SqlUrlScoring {
    url_id: u32,
    keyword_id: u32,
    pscore: f64,
    vscore: f64,
}

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn add_scoring(
        self: &Self,
        url_id: u32,
        keyword_id: u32,
        pscore: f64,
        vscore: f64,
        ) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"INSERT INTO url_scoring (url_id, keyword_id, pscore, vscore) VALUES (:url_id, :keyword_id, :pscore, :vscore)",
            params! {
                "url_id" => url_id,
                "keyword_id" => keyword_id,
                "pscore" => pscore,
                "vscore" => vscore,
            },
        );
        if let Err(err) = res {
            return Err(format!("insert.err: {}", err));
        }
        Ok(conn.last_insert_id() as u32)
    }
}