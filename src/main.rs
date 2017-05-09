extern crate regex;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    let input_path = "./tests/example.txt";
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let line = l.unwrap();
        // 正規表現を作る
        let pattern = Regex::new(r"([a-zA-Z])\s+->\s+(\*?[a-zA-Z])+").unwrap();
        // lineは 'String' だが， '&' をつけると '&str' になる
        let captures = pattern.captures(&line).unwrap();
        // 1つめのキャプチャを取得
        let first = captures.get(1).unwrap().as_str();
        // 表示
        println!("{}", first);
    }
}
