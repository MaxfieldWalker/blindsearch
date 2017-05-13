
pub struct BlindSearchStatus {
    status: String,
    result: BlindSearchResult,
}

pub enum SearchResult {
    Found,
    NotFound,
}

pub struct BlindSearchResult {
    statuses: Vec<BlindSearchStatus>,
    result: SearchResult,
}


impl BlindSearchStatus {
    fn to_string(&self) -> String {
        println!("{} L1: {}, L2: {}", self.loop_count, self.l1, self.l2);
    }
}
