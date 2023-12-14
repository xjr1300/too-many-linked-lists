use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
