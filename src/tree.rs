use std::rc::Rc;
use std::cell::RefCell;

type NodeRef = Rc<RefCell<_Node>>;

#[derive(Debug)]
struct _Node {
    value: String,
    children: Vec<NodeRef>,
}

// NodeRefは単なるタイプエイリアスなので
// タプル構造体を作ってまったく違う型にしてしまう。
pub struct Node(NodeRef);

impl Node {
    pub fn new(value: String) -> Node {
        let node = _Node {
            value: value,
            children: vec![],
        };
        Node(Rc::new(RefCell::new(node)))
    }

    pub fn add_child(&self, child_node: Node) {
        // ミュータブルな参照を得る
        // 'self.0' はタプル構造体の1つめの要素を取得している
        // つまり 'self.0' の型は 'Rc<RefCell<Node<T>>>' だが、
        // Rc<T>は '<Target=T>' を実装しているので自動的に
        // 'self.0' は 'RefCell<Node<T>>' 型に型強制される。
        // RefCell<T>の 'borrow_mut' 関数はラップしている値の
        // ミュータブルな参照を得る関数である。
        let mut me = self.0.borrow_mut();

        // 子ノードへの参照を得る
        // cloneしているが、子ノードへのポインタをクローンしているので
        // 値の複製が起こっているわけではない
        let child_node_ref = child_node.0.clone();
        me.children.push(child_node_ref);
    }

    pub fn stringify(&self) {
        // 自分の内容を表示
        self.inner_stringify(0);
    }

    fn inner_stringify(&self, depth: i32) {
        // depthの数だけスペースを生成する
        let spaces = (0..depth * 4).map(|_| " ").collect::<String>();

        let node_ref = self.0.borrow();

        println!("{}- {}", spaces, node_ref.value);
        if !node_ref.children.is_empty() {
            for child_ref in &node_ref.children {
                Node(child_ref.clone()).inner_stringify(depth + 1);
            }
        }
    }

    pub fn find_node(&self, value: &str) -> Option<Node> {
        let node_ref = self.0.borrow();
        if node_ref.value == value {
            return Some(Node(self.0.clone()));
        } else {
            for child_ref in &node_ref.children {
                let child_node = Node(child_ref.clone());
                match child_node.find_node(&value) {
                    Some(node) => {
                        return Some(node);
                    }
                    None => {
                        // 何もしない
                    }
                }
            }

            return None;
        }
    }
}

// fn main() {
//     let root_node = Node::new("S".to_string());

//     let a_node = Node::new("a".to_string());
//     let b_node = Node::new("b".to_string());

//     root_node.add_child(&a_node);
//     root_node.add_child(&b_node);

//     root_node.stringify();

//     match root_node.find_node("d") {
//         Some(node) => println!("FOUND"),
//         None => println!("NOT FOUND"),
//     }
// }
