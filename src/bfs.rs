// 横型探索(幅優先探索, Breadth-First Search)

use tree::*;
use blindsearch::*;

pub fn blind_search_bfs(root_node: &Node, goals: &Vec<String>) -> BlindSearchResult {
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

    // Step 2:
    // L1が空なら探索は失敗
    if l1.len() == 0 {
        // 探索失敗
        vec![BlindSearchStatus::new(str_status, SearchResult::NotFound)]
    } else {
        // Step 3:
        // L1の先頭の節点 n を取り除き，L2に入れる
        let n = l1.remove(0);
        l2.push(n.ref_clone());

        // Step 4:
        // n が目標節点ならば探索成功
        if goals.contains(&n.name()) {
            // 探索成功
            vec![BlindSearchStatus::new(str_status, SearchResult::Found)]
        } else {
            // Step 5:
            // n が子節点を持つならば展開して得られた子接点を順序そのままに
            // L1の末尾に入れる
            if n.has_child_node() {
                for child_node in n.children() {
                    l1.push(child_node.ref_clone());
                }
            }

            let this_status = BlindSearchStatus::new(str_status, SearchResult::NotFound);

            // Step 2に戻る
            let mut rest_status = dig(l1, l2, loop_count + 1, goals);

            rest_status.insert(0, this_status);
            rest_status
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

        let result = blind_search_bfs(&root_node, &vec![]);
        let as_str: Vec<String> = result
            .statuses
            .iter()
            .map(|x| format!("{:?}", x))
            .collect();

        let expected = vec!["NotFound loop_count: 0 L1[S] L2[]",
                            "NotFound loop_count: 1 L1[a, b] L2[S]",
                            "NotFound loop_count: 2 L1[b, c, d] L2[S, a]",
                            "NotFound loop_count: 3 L1[c, d, e, f] L2[S, a, b]",
                            "NotFound loop_count: 4 L1[d, e, f] L2[S, a, b, c]",
                            "NotFound loop_count: 5 L1[e, f] L2[S, a, b, c, d]",
                            "NotFound loop_count: 6 L1[f, g, h] L2[S, a, b, c, d, e]",
                            "NotFound loop_count: 7 L1[g, h] L2[S, a, b, c, d, e, f]",
                            "NotFound loop_count: 8 L1[h] L2[S, a, b, c, d, e, f, g]",
                            "NotFound loop_count: 9 L1[] L2[S, a, b, c, d, e, f, g, h]"];

        assert_eq!(expected, as_str);
    }
}
