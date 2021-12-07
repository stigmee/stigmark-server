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

#[derive(Debug, PartialEq)]
pub struct SqlStigmeeEvents {
    event_id: u32,
    event_date: NaiveDateTime,
    event_type: u32,
    event_desc: String,
    event_arg1: u32,
    event_arg2: u32,
    event_arg3: u32,
    event_arg4: String,
}

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn add_event<S: Into<String>>(
        self: &Self,
        event_type: u32,
        event_desc: String,
        event_arg1: u32,
        event_arg2: u32,
        event_arg3: u32,
        event_arg4: String,
    ) -> Result<u32, String> {
        let conn = &mut self.pool.get_conn().expect("sql: could not connect");
        let res = conn.exec_drop(
            r"INSERT INTO stigmee_events (event_type, event_desc, event_arg1, event_arg2, event_arg3, event_arg4) VALUES (:event_type, :event_desc, :event_arg1, :event_arg2, :event_arg3, :event_arg4)",
            params! {
                "event_type" => event_type,
                "event_desc" => event_desc,
                "event_arg1" => event_arg1,
                "event_arg2" => event_arg2,
                "event_arg3" => event_arg3,
                "event_arg4" => event_arg4,
            },
        );
        if let Err(err) = res {
            return Err(format!("insert.err: {}", err));
        }
        Ok(conn.last_insert_id() as u32)
    }
}
