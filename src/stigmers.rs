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
