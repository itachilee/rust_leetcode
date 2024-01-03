use itertools::Itertools;

fn main() {}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;

    // 遍历链表
    while let Some(ref mut node) = current {
        let mut next_node = node.next.take();
        while let Some(ref mut next) = next_node {
            if node.val == next.val {
                next_node = next.next.take(); // 删除重复节点
            } else {
                break;
            }
        }
        node.next = next_node;
        current = node.next.take();
    }

    current
}
mod tests {

    use super::*;
    #[test]

    fn test_insert() {}
}
