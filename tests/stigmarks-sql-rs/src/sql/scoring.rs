#[derive(Debug, PartialEq)]
pub struct SqlUrlScoring {
    url_id: u32,
    keyword_id: u32,
    pscore: f64,
    vscore: f64,
}
