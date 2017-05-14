
#[derive(Debug)]
pub struct BlindSearchStatus {
    // ループ回数とL1とL2の情報
    status: String,
    // 目標節点が見つかったか見つからなかったか
    result: SearchResult,
}

impl BlindSearchStatus {
    pub fn new(status: String, result: SearchResult) -> BlindSearchStatus {
        BlindSearchStatus { status, result }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SearchResult {
    Found,
    NotFound,
}

#[derive(Debug)]
pub struct BlindSearchResult {
    pub statuses: Vec<BlindSearchStatus>,
    pub result: SearchResult,
}
