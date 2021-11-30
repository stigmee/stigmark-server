use mysql::{Opts, Pool};

pub struct SqlStigmarksDB {
    conn: mysql::PooledConn,
}

mod users;

#[allow(dead_code)]
impl SqlStigmarksDB {
    pub fn new(db_name: &str, db_pass: &str) -> Self {
        let url = format!("mysql://{}:{}@localhost:3306/stigmarks", db_name, db_pass);
        let opts = Opts::from_url(url.as_str()).unwrap();
        let pool = Pool::new(opts).unwrap();
        let conn = pool.get_conn().unwrap();
        Self{
            conn,
        }
    }
}
