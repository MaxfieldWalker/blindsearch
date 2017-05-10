extern crate regex;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

mod tree;

use tree;

// 間違いないだろう
fn capture<'a>(s: &'a String) -> regex::Captures<'a> {
    let pattern = Regex::new(r"([a-zA-Z])\s+->\s+((\*?[a-zA-Z])+)").unwrap();
    pattern.captures(s).unwrap()
}

// 1行を与えて、その節点が目標節点かどうかと節点名を返す
fn getInfo<'a>(line: &'a String) -> (String, Vec<(bool, String)>) {
    let captures = capture(line);
    let first = captures.get(1).unwrap().as_str();
    let mut children: Vec<(bool, String)> = vec![];

    let rest = captures.get(2).unwrap().as_str();


    let mut index = 0;
    while index < rest.len() {
        // 一文字だけ得る
        let sub = &rest[index..index + 1];
        let mut node_name: &str = sub;
        let mut is_goal_node = false;
        index += 1;

        // '*'がついている、つまり目標節点の場合は
        // さらにポインタを1すすめる
        if sub == "*" {
            // 部分文字列を取得している
            node_name = &rest[index..index + 1];
            index += 1;
            is_goal_node = true;
        }

        children.push((is_goal_node, node_name.to_string()));
    }

    (first.to_string(), children)
}

fn f(line: String, root_node: &mut tree::Node, goals: &mut Vec<String>) {
    let (node_name, children) = getInfo(&line);

    let samekey: String = node_name.clone();

    // 仮の親ノード
    // &mut参照する可能性があるのでmutをつける必要がある
    let mut node = tree::Node::new(samekey.to_string());


    // 間違い無し
    let mut parent_node: &mut tree::Node = match root_node.findNode(&node_name) {
        Some(n) => {

            // let a: () = n;
            n
        }
        None => {
            let n: &mut tree::Node = &mut node;
            n
        }
    };


    // ルートノードを取得する方法はまた考えよう
    // ひとまずディクショナリからキーを検索する方法で回避する

    for (isGoalNode, name) in children {

        let samekey: String = name.clone();
        let child_node = tree::Node::new(samekey.to_string());

        // 親ノードに子ノードを追加する
        parent_node.add_child(child_node);

        if isGoalNode {
            // ここでnameの所有権が移る
            goals.push(name);
        }

    }
}

fn main() {
    let root_node = TreeNode::new("Hello".to_string());

    let a_node = TreeNode::new("a".to_string());

    let b_node = TreeNode::new("b".to_string());

    root_node.add_child(&a_node);
    root_node.add_child(&b_node);

    // root_nodeの型は 'Node<String>'
    let a: () = root_node;


    // let input_path = "./tests/example.txt";
    // let file = File::open(input_path).unwrap();
    // let mut reader = BufReader::new(file);

    // // 目標節点を格納する配列
    // let mut goals: Vec<String> = vec![];

    // // 1行目を読む
    // let mut first_line = String::new();
    // reader.read_line(&mut first_line);

    // // パースする
    // let (node_name, _) = getInfo(&first_line);

    // // ルートノードを作成する
    // let mut root_node = tree::Node::new(node_name);

    // f(first_line, &mut root_node, &mut goals);

    // for l in reader.lines() {
    //     let line = l.unwrap();
    //     f(line, &mut root_node, &mut goals);
    // }
}
