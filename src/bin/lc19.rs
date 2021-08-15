use std::borrow::Borrow;

#[derive(PartialEq, Eq, Clone, Debug)]
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

fn main() {}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut dump_head = Box::new(ListNode {
        next: head.take(),
        val: -1,
    });
    let mut fast_node = dump_head.clone();
    let mut slow_node = &mut dump_head;
    for _ in 0..n {
        if let Some(node) = fast_node.next {
            fast_node = node;
            continue;
        }
        return dump_head.next;
    }
    while let Some(next_fast_node) = fast_node.next {
        fast_node = next_fast_node;
        if let Some(ref mut next_slow_node) = slow_node.next {
            slow_node = next_slow_node;
        } else {
            return dump_head.next;
        }
    }
    if let Some(ref mut node) = slow_node.next {
        (*slow_node).next = node.next.take();
    }
    dump_head.next
}
