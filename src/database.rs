// 
//  Stigmee: A 3D browser and decentralized social network.
//  Copyright 2021 Philippe Anel <zexigh@gmail.com>
// 
//  This file is part of Stigmee.
// 
//  Project : Stigmark
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

// thread stuff
use std::sync::mpsc;

// file stuff
use std::fs::{File, create_dir_all};
use std::io::BufReader;
use std::path::Path;

// json stuff
use serde::{Deserialize, Serialize};

use crate::stigmarks::StigmarkData;

// backup stuff
use chrono::{Datelike, Timelike, Utc};
const MAX_UPDATES_BEFORE_SAVING: u32 = 5;

#[derive(Serialize, Deserialize)]
struct StigmarkDB {
    groups: Vec<StigmarkGroup>,
}

#[derive(Serialize, Deserialize)]
struct StigmarkGroup {
    gid: u32,
    urls: Vec<StigmarkURL>,
    stigmarks: Vec<StigmarkEntry>,
}

#[derive(Serialize, Deserialize)]
struct StigmarkURL {
    uid: u32,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct StigmarkEntry {
    user: u32,
    urls: Vec<u32>,
    keywords: Vec<String>,
    // todo: add hidden/publish bool ?
}

fn read_db_from_json(name: &str) -> Result<StigmarkDB, String> {
    match File::open(name) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let res: Result<StigmarkDB, serde_json::Error> = serde_json::from_reader(reader);
            match res {
                Ok(db) => Ok(db),
                Err(err) => Err(format!("{}", err)),
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}

fn write_db_to_json(name: &str, db: &StigmarkDB) {
    let file = File::create(name).unwrap();
    serde_json::to_writer(file, &db).unwrap();
}

const STIGMARK_FILE_NAME: &str = "data/stigmarks.json";

// handles json database
pub fn save_stigmarks_service(rx: mpsc::Receiver<StigmarkData>) {
    let p = Path::new(STIGMARK_FILE_NAME);
    let d = p.parent().unwrap();
    create_dir_all(d).unwrap();

    let mut stigmark_db = match read_db_from_json(STIGMARK_FILE_NAME) {
        Ok(stigmark_db) => stigmark_db,
        Err(_) => {
            let group0 = StigmarkGroup {
                gid: 1,
                urls: vec![],
                stigmarks: vec![],
            };
            StigmarkDB {
                groups: vec![group0],
            }
        }
    };

    let mut updates_before_backup = MAX_UPDATES_BEFORE_SAVING;
    loop {
        let mark = rx.recv().unwrap();

        updates_before_backup -= 1;
        if updates_before_backup == 0 {
            updates_before_backup = MAX_UPDATES_BEFORE_SAVING;
            let now = Utc::now();
            let path = Path::new(STIGMARK_FILE_NAME);
            let parent = path.parent().unwrap();
            let stem = path.file_stem().unwrap();
            let ext = path.extension().unwrap();
            let backup_name = format!(
                "{:?}/{:?}-{}-{}-{}-{}-{}-{}.{:?}",
                parent,
                stem,
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
                ext,
            );
            write_db_to_json(backup_name.as_str(), &stigmark_db);
        }

        let mut urls = vec![]; // urls uids
        for u in mark.urls {
            // look if url already in db
            let (mut uid, mut found) = (0, false);
            for ku in &stigmark_db.groups[0].urls {
                if u == ku.url {
                    uid = ku.uid;
                    found = true;
                };
            }
            // if not in db, add it
            if !found {
                uid = stigmark_db.groups[0].urls.len() as u32;
                stigmark_db.groups[0].urls.push(StigmarkURL {
                    uid: uid,
                    url: u.clone(),
                });
                println!("stigmark: added url {}:{}", uid, u);
            }
            urls.push(uid);
        }

        // add mark to db
        stigmark_db.groups[0].stigmarks.push(StigmarkEntry {
            user: mark.user,
            urls: urls,
            keywords: mark.keys,
        });

        write_db_to_json(STIGMARK_FILE_NAME, &stigmark_db);
    }
}
