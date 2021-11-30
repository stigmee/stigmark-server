use mysql::prelude::{Queryable};
use mysql::{params};

use crate::SqlUser;
pub use crate::sql::SqlStigmarksDB;

#[allow(dead_code)]
impl SqlStigmarksDB {
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
