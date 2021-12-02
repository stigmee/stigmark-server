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

use std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stigmers {
    users: Vec<Stigmer>,
}

#[derive(Serialize, Deserialize)]
pub struct Stigmer {
    name: String,
    email: String,
    hash: Vec<u8>,
    verified: bool,
    disabled: bool,
    friends: Vec<u32>,
}

pub struct StigmerService {
    path: String,
    // todo: RWMutex
}

#[allow(dead_code)]
impl StigmerService {
    pub fn new(path: &str) -> StigmerService {
        StigmerService{
            path: String::from(path),
        }
    }

    pub fn reload(self: &Self) {
        // todo: reload from json file: must acquire exclusive mutex
    }

    pub fn get_path(self: &Self) -> String {
        self.path.clone()
    }

    pub fn add_user(self: &Self, _name: String, _email: String, _passwd: String) {
    }

    pub fn find_user_by_email(self: &Self, _email: String) -> u32 {
        0
    }

    pub fn check_password(self: &Self, _user_id: u32, _passwd: String) -> bool {
        false
    }

    pub fn get_user_name(self: &Self, _user_id: u32) -> String {
        String::from("todo")
    }

    pub fn set_user_name(self: &Self, _user_id: u32, _email: String) {
    }

    pub fn get_user_email(self: &Self, _user_id: u32) -> String {
        String::from("todo")
    }

    pub fn set_user_email(self: &Self, _user_id: u32, _email: String) {
    }

    pub fn set_user_password(self: &Self, _user_id: u32, _email: String) {
    }

    pub fn add_friend(self: &Self, _user_id: u32, _friend_id: u32) {
    }

    pub fn remove_friend(self: &Self, _user_id: u32, _friend_id: u32) {
    }

    pub fn validate_user(self: &Self, _user_id: u32) {
    }

    pub fn enable_user(self: &Self, _user_id: u32) {
    }

    pub fn disable_user(self: &Self, _user_id: u32) {
    }
}
