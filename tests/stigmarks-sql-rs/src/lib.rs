use mysql::chrono::NaiveDateTime;

pub mod sql;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlUser {
    id: u32,
    name: String,
    email: String,
    // hidden: hash: Vec<u8>,
    creation_date: NaiveDateTime,
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlKeyword {
//     id: u32,
//     keyword: String,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlUrl {
//     id: u32,
//     url: String,
// }

#[derive(Debug, PartialEq, Eq)]
pub struct SqlCollection {
    id: u32,
    user_id: u32,
    creation_date: mysql::chrono::NaiveDateTime,
    hidden: bool,
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlKeywordList {
//     collection_id: u32,
//     keyword_id: u32,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct SqlUrlList {
//     collection_id: u32,
//     url_id: u32,
// }

#[derive(Debug, PartialEq)]
pub struct SqlUrlScoring {
    url_id: u32,
    keyword_id: u32,
    pscore: f64,
    vscore: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // todo
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
