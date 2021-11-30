use stigmarks::sql::SqlStigmarksDB;

const DB_USER: &str = "stigmark";
const DB_PASS: &str = "yAfisEra";

fn main() {
    let mut stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);
    let all_users = stigmarks_db.get_all_users();
    match all_users {
        Ok(users) => println!("{:?}", users),
        Err(err) => eprintln!("get_all_users failed: {}", err),
    }

    let user_1 = stigmarks_db.get_user_by_id(1);
    match user_1 {
        Ok(user) => println!("{:?}", user),
        Err(err) => eprintln!("get_user_by_id failed: {}", err),
    }
}
