// Definition for singly-linked list.
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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p = head.as_mut();
    while let Some(node) = p {
        if node.next.as_ref().is_some() && node.val == node.next.as_ref().unwrap().val {
            let tmp = node.next.as_mut().unwrap().next.take();
            node.next = tmp;
            p = Some(node)
        } else {
            p = node.next.as_mut();
        }
    }

    head
}
