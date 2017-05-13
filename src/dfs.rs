// 縦型探索(深さ優先探索, Depth-First Search)

// mod blindsearch;

use tree::Node;
use blindsearch::*;

fn blind_search_dfs(root_node: &Node, goals: &Vec<String>) -> BliendSearchStatus {
    let mut l1: Vec<&Node> = vec![];
    let mut l2: Vec<&Node> = vec![];

    // Step 1:
    // 出発節点をL1に入れる
    l1.push(root_node);
    dig(&mut l1, &mut l2, 1)
}

fn dig(l1: &mut Vec<&Node>, l2: &mut Vec<&Node>, loop_count: u32) -> Vec<BlindSearchResult> {
    let str_status = vec![format!("{} | {:?} | {:?}", loop_count, l1, l2)];

    // Step2:
    // L1が空ならば探索失敗，終了
    if l1.len() == 0 {
        vec![BlindSearchStatus {
                 status: str_status,
                 result: SearchResult::NotFound,
             }]
    } else {
        // Step 3:
        // L1の先頭の節点 n を取り出し，L2に入れる
        let first_l1: &Node = l1[0];
        l2.push(first_l1);

        // Step 4:
        // n が目標節点ならば探索成功
        if goals.contains(first_l1) {
            // 探索成功

        } else {
            // Step 5:
            // n が展開できるならば，得られた子接点を順序そのままに
            // L1の先頭に入れる
            if first_l1.has_child_node() {
                for child in &first_l1.0.borrow().children.iter().rev() {
                    let clone = child.clone();
                    l1.insert(0, clone);
                }
            }

            let this_status = vec![BlindSearchStatus {
                                       status: str_status,
                                       result: SearchResult::NotFound,
                                   }];


            // Step 2に戻る
            let rest_statuses = dig(l1, l2, loop_count + 1);

            result.extend(&rest_result)
        }
    }
}
