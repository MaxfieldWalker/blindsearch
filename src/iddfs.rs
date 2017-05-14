// 反復深化深さ優先探索(Iterative Deepening Depth-First Search)

use tree::*;
use blindsearch::*;

pub fn blind_search_iddfs(root_node: &Node,
                          goals: &Vec<String>,
                          cutoff_threshold: u32)
                          -> BlindSearchResult {
    let mut l1: Vec<Node> = vec![];
    let mut l2: Vec<Node> = vec![];

    // Step 1:
    // cutoff の初期値を1とする
    let mut cutoff: u32 = 1;

    // Step 2'
    // 出発節点をL1に入れる
    l1.push(root_node.ref_clone());

    let statuses = dig(&mut l1,
                       &mut l2,
                       cutoff,
                       cutoff_threshold,
                       0,
                       0,
                       root_node,
                       goals);

    BlindSearchResult {
        statuses: statuses,
        result: SearchResult::Found,
    }
}


fn dig(l1: &mut Vec<Node>,
       l2: &mut Vec<Node>,
       cutoff: u32,
       cutoff_threshold: u32,
       loop_count: u32,
       current_depth: u32,
       root_node: &Node,
       goals: &Vec<String>)
       -> Vec<BlindSearchStatus> {
    let str_status: String = format!("loop_count: {} L1{:?} L2{:?}", loop_count, l1, l2);
    if cutoff > cutoff_threshold {
        // 探をを打ち切り失敗とする
        vec![]
    } else {
        if l1.len() == 0 {
            // Step 2, 3:
            // L1が空ならば cutoff を1増やし
            // 出発節点をL1に入れる
            let this_status = BlindSearchStatus::new(str_status, SearchResult::NotFound);

            l1.push(root_node.ref_clone());

            // l2 は空にする
            let mut l2_empty = vec![];
            let mut rest_status = dig(l1,
                                      &mut l2_empty,
                                      cutoff + 1,
                                      cutoff_threshold,
                                      loop_count + 1,
                                      0,
                                      root_node,
                                      goals);
            rest_status.insert(0, this_status);
            rest_status
        } else {
            // Step 4:
            // L1の先頭の節点を取り出し，L2に入れる
            let n = l1.remove(0);
            l2.push(n.ref_clone());

            // Step 5:
            // n が目標節点ならば探索成功
            if goals.contains(&n.name()) {
                // 成功
                vec![BlindSearchStatus::new(str_status, SearchResult::Found)]
            } else {
                // Step 6:
                // n が展開可能 かつ nの深さが cutoff より小さい場合
                // n 展開して得られた子接点を順序そのままにL1の先頭に入れる
                let digin = n.has_child_node() && current_depth < cutoff;
                let depth = if digin {
                    current_depth + 1
                } else {
                    current_depth
                };

                let depth = root_node.find_node(&n.name()).unwrap().1 + 1;

                if n.has_child_node() && depth < cutoff {
                    for child_node in n.children().iter().rev() {
                        l1.insert(0, child_node.ref_clone());
                    }
                }

                // Step 3 へ
                let this_status = BlindSearchStatus::new(str_status, SearchResult::NotFound);
                let mut rest_status = dig(l1,
                                          l2,
                                          cutoff,
                                          cutoff_threshold,
                                          loop_count + 1,
                                          depth,
                                          root_node,
                                          goals);
                rest_status.insert(0, this_status);

                rest_status
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core;

    #[test]
    fn test_dfs() {
        let root_node = core::build_tree_from_input(&"./tests/example.txt".to_string());

        let result = blind_search_iddfs(&root_node, &vec![], 4);
        let as_str: Vec<String> = result
            .statuses
            .iter()
            .map(|x| format!("{:?}", x))
            .collect();

        for s in &result.statuses {
            println!("{:?}", s);
        }

        let expected = vec!["NotFound loop_count: 0 L1[S] L2[]",
                            "NotFound loop_count: 1 L1[] L2[S]",
                            "NotFound loop_count: 2 L1[S] L2[]",
                            "NotFound loop_count: 3 L1[a, b] L2[S]",
                            "NotFound loop_count: 4 L1[b] L2[S, a]",
                            "NotFound loop_count: 5 L1[] L2[S, a, b]",
                            "NotFound loop_count: 6 L1[S] L2[]",
                            "NotFound loop_count: 7 L1[a, b] L2[S]",
                            "NotFound loop_count: 8 L1[c, d, b] L2[S, a]",
                            "NotFound loop_count: 9 L1[d, b] L2[S, a, c]",
                            "NotFound loop_count: 10 L1[b] L2[S, a, c, d]",
                            "NotFound loop_count: 11 L1[e, f] L2[S, a, c, d, b]",
                            "NotFound loop_count: 12 L1[f] L2[S, a, c, d, b, e]",
                            "NotFound loop_count: 13 L1[] L2[S, a, c, d, b, e, f]",
                            "NotFound loop_count: 14 L1[S] L2[]",
                            "NotFound loop_count: 15 L1[a, b] L2[S]",
                            "NotFound loop_count: 16 L1[c, d, b] L2[S, a]",
                            "NotFound loop_count: 17 L1[d, b] L2[S, a, c]",
                            "NotFound loop_count: 18 L1[b] L2[S, a, c, d]",
                            "NotFound loop_count: 19 L1[e, f] L2[S, a, c, d, b]",
                            "NotFound loop_count: 20 L1[g, h, f] L2[S, a, c, d, b, e]",
                            "NotFound loop_count: 21 L1[h, f] L2[S, a, c, d, b, e, g]",
                            "NotFound loop_count: 22 L1[f] L2[S, a, c, d, b, e, g, h]",
                            "NotFound loop_count: 23 L1[] L2[S, a, c, d, b, e, g, h, f]"];

        assert_eq!(expected, as_str);
    }
}
