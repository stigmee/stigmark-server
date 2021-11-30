use stigmarks::sql::SqlStigmarksDB;

const DB_USER: &str = "stigmark";
const DB_PASS: &str = "yAfisEra";

fn main() {
    let mut stigmarks_db = SqlStigmarksDB::new(DB_USER, DB_PASS);

    println!("get_all_users");
    let all_users = stigmarks_db.get_all_users();
    match all_users {
        Ok(users) => println!("{:?}", users),
        Err(err) => eprintln!("get_all_users failed: {}", err),
    }

    println!("get_user_by_id");
    let user_1 = stigmarks_db.get_user_by_id(1);
    match user_1 {
        Ok(user) => println!("{:?}", user),
        Err(err) => eprintln!("get_user_by_id failed: {}", err),
    }

    println!("get_all_collections");
    let all_collections = stigmarks_db.get_all_collections();
    match all_collections {
        Ok(collections) => println!("{:?}", collections),
        Err(err) => eprintln!("get_all_collections failed: {}", err),
    }

    println!("get_collection_by_id");
    let collection_1 = stigmarks_db.get_collection_by_id(1);
    match collection_1 {
        Ok(collection) => println!("{:?}", collection),
        Err(err) => eprintln!("get_collection_by_id failed: {}", err),
    }
}
