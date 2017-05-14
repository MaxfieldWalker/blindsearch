use std::fmt;

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

// Node構造体のDebugトレイトの実装
impl fmt::Debug for BlindSearchStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.result, self.status)
    }
}
