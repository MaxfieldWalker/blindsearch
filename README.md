## blindsearch

[![Build Status](https://travis-ci.org/MaxfieldWalker/blindsearch.svg?branch=master)](https://travis-ci.org/MaxfieldWalker/blindsearch)



3種類のブラインド探索

- 縦型探索(深さ優先) ([ソース](src/dfs.rs))
- 横型探索(幅優先) ([ソース](src/bfs.rs))
- 反復深化探索(ハイブリッド) ([ソース](src/iddfs.rs))


## 仕様

- 節点名はアルファベット1文字で`a-zA-Z`である


### 入力仕様
探索木の表現は以下の通り。

<節点> -> <子接点>(<子接点>...)
ただし，一行目の<節点>がルートノードである。
また，目標節点には`*`を節点名の前につける。

```
例: tree.txt

S -> ab
a -> cd
b -> ef
e -> *gh
```
