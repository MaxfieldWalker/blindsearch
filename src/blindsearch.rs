
pub struct BlindSearchStatus {
    // ループ回数
    loop_count: u32,

    // L1 nodes
    l1: Vec<String>,

    // L2 nodes
    l2: Vec<String>,
}

pub struct BlindSearchResult {
    statuses: Vec<BlindSearchStatus>,
}


impl BlindSearchStatus {
    fn to_string(&self) -> String {
        println!("{} L1: {}, L2: {}", self.loop_count, self.l1, self.l2);
    }
}
