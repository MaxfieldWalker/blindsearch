// 縦型探索(深さ優先探索, Depth-First Search)

// mod blindsearch;
use tree::*;
use blindsearch::*;

pub fn blind_search_dfs(root_node: &Node, goals: &Vec<String>) -> BlindSearchResult {
    let mut l1: Vec<Node> = vec![];
    let mut l2: Vec<Node> = vec![];

    // Step 1:
    // 出発節点をL1に入れる
    l1.push(root_node.ref_clone());
    let statuses = dig(&mut l1, &mut l2, 0, goals);
    BlindSearchResult {
        statuses: statuses,
        result: SearchResult::Found,
    }
}


fn dig(l1: &mut Vec<Node>,
       l2: &mut Vec<Node>,
       loop_count: u32,
       goals: &Vec<String>)
       -> Vec<BlindSearchStatus> {
    let str_status: String = format!("loop_count: {} L1{:?} L2{:?}", loop_count, l1, l2);

    // Step2:
    // L1が空ならば探索失敗，終了
    if l1.len() == 0 {
        vec![BlindSearchStatus::new(str_status, SearchResult::NotFound)]
    } else {
        // Step 3:
        // L1の先頭の節点 n を取り出し，L2に入れる

        // remove関数を使うことで先頭要素を取り出し，
        // 所有権も配列側から奪うことが出来る
        let first_l1: Node = l1.remove(0);
        l2.push(first_l1.ref_clone());

        // Step 4:
        // n が目標節点ならば探索成功
        if goals.contains(&first_l1.name()) {
            // 探索成功
            vec![BlindSearchStatus::new(str_status, SearchResult::Found)]
        } else {
            // Step 5:
            // n が展開できるならば，得られた子接点を順序そのままに
            // L1の先頭に入れる
            if first_l1.has_child_node() {
                for child_node in first_l1.children().iter().rev() {
                    l1.insert(0, child_node.ref_clone());
                }
            }

            let this_status = BlindSearchStatus::new(str_status, SearchResult::NotFound);

            // Step 2に戻る
            let mut rest_statuses: Vec<BlindSearchStatus> = dig(l1, l2, loop_count + 1, goals);

            rest_statuses.insert(0, this_status);

            rest_statuses
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

        let result = blind_search_dfs(&root_node, &vec![]);
        let as_str: Vec<String> = result
            .statuses
            .iter()
            .map(|x| format!("{:?}", x))
            .collect();

        let expected = vec!["NotFound loop_count: 0 L1[S] L2[]",
                            "NotFound loop_count: 1 L1[a, b] L2[S]",
                            "NotFound loop_count: 2 L1[c, d, b] L2[S, a]",
                            "NotFound loop_count: 3 L1[d, b] L2[S, a, c]",
                            "NotFound loop_count: 4 L1[b] L2[S, a, c, d]",
                            "NotFound loop_count: 5 L1[e, f] L2[S, a, c, d, b]",
                            "NotFound loop_count: 6 L1[g, h, f] L2[S, a, c, d, b, e]",
                            "NotFound loop_count: 7 L1[h, f] L2[S, a, c, d, b, e, g]",
                            "NotFound loop_count: 8 L1[f] L2[S, a, c, d, b, e, g, h]",
                            "NotFound loop_count: 9 L1[] L2[S, a, c, d, b, e, g, h, f]"];

        assert_eq!(expected, as_str);
    }
}
