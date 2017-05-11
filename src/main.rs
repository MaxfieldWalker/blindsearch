extern crate regex;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

mod tree;

use tree::Node;

fn capture<'a>(s: &'a String) -> regex::Captures<'a> {
    let pattern = Regex::new(r"([a-zA-Z])\s+->\s+((\*?[a-zA-Z])+)").unwrap();
    pattern.captures(s).unwrap()
}

// 1行を与えて、その節点が目標節点かどうかと節点名を返す
fn parse_line<'a>(line: &'a String) -> (String, Vec<(bool, String)>) {
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

fn build_tree(line: String, root_node: &Node, goals: &mut Vec<String>) {
    let (node_name, children) = parse_line(&line);

    let parent_node: Node = match root_node.find_node(&node_name) {
        Some(n) => n,
        None => Node::new(node_name.clone().to_string()),
    };

    // ルートノードを取得する方法はまた考えよう
    // ひとまずディクショナリからキーを検索する方法で回避する

    for (is_goal_node, name) in children {
        let child_node = Node::new(name.clone().to_string());

        // 親ノードに子ノードを追加する
        parent_node.add_child(child_node);

        if is_goal_node {
            // ここでnameの所有権が移る
            goals.push(name);
        }
    }
}

fn main() {
    let input_path = "./tests/example.txt";
    let file = File::open(input_path).unwrap();
    let mut reader = BufReader::new(file);

    // 目標節点を格納する配列
    let mut goals: Vec<String> = vec![];

    // 1行目を読む
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    // パースする
    let (node_name, _) = parse_line(&first_line);

    // ルートノードを作成する
    let root_node = Node::new(node_name);


    build_tree(first_line, &root_node, &mut goals);

    // 上でread_lineを1回呼んでいるので
    // 2行目以降がイテレートされる
    for l in reader.lines() {
        let line = l.unwrap();
        build_tree(line, &root_node, &mut goals);
    }

    root_node.stringify();
}
