## blindsearch

[![Build Status](https://travis-ci.org/MaxfieldWalker/blindsearch.svg?branch=master)](https://travis-ci.org/MaxfieldWalker/blindsearch)
[![codecov](https://codecov.io/gh/MaxfieldWalker/blindsearch/branch/master/graph/badge.svg)](https://codecov.io/gh/MaxfieldWalker/blindsearch)


3種類のブラインド探索

- 縦型探索(深さ優先) [ソースへ移動](src/dfs.rs)
- 横型探索(幅優先) [ソースへ移動](src/bfs.rs)
- 反復深化深さ優先探索(ハイブリッド) [ソースへ移動](src/iddfs.rs)


## 仕様

- 節点名はアルファベット1文字で`a-zA-Z`である


### 入力仕様
```
<節点> -> <子接点>(<子接点>...)
```

```
例: tree.txt

S -> ab
a -> cd
b -> ef
e -> *gh
```

探索木の表現は上記の通り。
ただし，一行目の<節点>がルートノードである。
また，目標節点には`*`を節点名の前につける。
