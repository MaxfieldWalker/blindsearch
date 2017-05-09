pub struct Node<'a> {
    pub name: &'a str,
    children: Vec<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(n: &'a str) -> Node<'a> {
        Node {
            name: n,
            children: vec![],
        }
    }

    pub fn add_child4(&mut self, name: &'a str) {
        let new_node = Node::new(name);
        self.children.push(Box::new(new_node));
    }

    pub fn add_child2(&mut self, node: Box<Node<'a>>) {
        self.children.push(node);
    }

    pub fn add_child(&mut self, node: Node<'a>) {
        self.children.push(Box::new(node));
    }

    // - 自身の内容
    //     - 子供
    //         - さらに子供
    //     - 子供
    //     - 子供
    pub fn stringify(&self) {
        self.inner_stringify(0);
    }

    fn inner_stringify(&self, depth: u32) {
        // depthの数だけスペースを生成している
        let spaces = (0..depth).map(|_| " ").collect::<String>();
        println!("{}- {}", spaces, self.name);
        if !self.children.is_empty() {
            // ここでも借用する必要がある
            let children = &self.children;
            // childはBox<Node<'a>>なのでrefキーワードで参照する必要がある
            for ref child in children {
                child.inner_stringify(depth + 4);
            }
        }
    }
}

// enum: 列挙型とは言うが
// Union Type(直和型)に近い e.g. str | Nil
pub enum Tree {
    // 自身の内容と次の要素へのポインタを持つ
    // タプル構造体である(Pairという名前を付けているが何でもいい)
    Node,

    // 空であることを表す
    // フィールドを持たない 'Unit-like struct' である
    Nil,
}
