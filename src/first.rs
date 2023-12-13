use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // 完了しており、何もしない
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // 末尾再帰で問題ない
//             }
//         }
//     }
// }

// impl Drop for Box<self::Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_sizes() {
        println!("size of List: {}", std::mem::size_of::<List>()); // 8byte
        println!("size of Link: {}", std::mem::size_of::<Link>()); // 8byte
        println!("size of Node: {}", std::mem::size_of::<Node>()); // 16byte
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // 空のリストが正しく振る舞うか確認
        assert_eq!(list.pop(), None);

        // リストを生成
        list.push(1);
        list.push(2);
        list.push(3);

        // 通常の取り出しを確認
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // 何も破壊されていないことを確認するために、さらにプッシュ
        list.push(4);
        list.push(5);

        // 通常の取り出しを確認
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // 完全に使い果たしたか確認
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
