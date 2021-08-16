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

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // {
    let mut head = head;
    let mut p = head.as_mut();
    {
        while let Some(node) = p {
            if node.next.as_ref().is_some() {
                p = node.next.as_mut();
            } else {
                node.next.as_mut().unwrap().next = head.take();
            }
        }
    }
    // }
    head
}
