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

use mysql::{Opts, Pool};

pub struct SqlStigmarksDB {
    // conn: mysql::PooledConn,
    pool: mysql::Pool
}

pub mod users;
pub mod collections;

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn new(db_name: &str, db_pass: &str) -> Self {
        let url = format!("mysql://{}:{}@localhost:3306/stigmarks", db_name, db_pass);
        let opts = Opts::from_url(url.as_str()).expect("sql: failed get opts from url");
        let pool = Pool::new(opts).expect("sql: could create pool");
        // let conn = pool.get_conn().expect("sql: could not connect");
        // Self{
        //     conn,
        // }
        Self {
            pool
        }
    }

    pub fn init() {
        // todo: init sql table from '../../../../sql/stigmarks.sql'
    }
}
