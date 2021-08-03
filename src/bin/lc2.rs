#[warn(unused)]
#[warn(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
fn main(){

}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{

    let mut head= ListNode::new(0);
    let mut flag=0;
    let mut p1=l1.as_ref();
    let mut p2=l2.as_ref();
    let mut p=&mut  head;
    while p1.is_some()||p2.is_some() {
        let l1_val=p1.map_or(0, |node| node.val);
        let l2_val=p2.map_or(0, |node| node.val);
        
        let sum=l1_val+l2_val+flag;
        let rst= sum%10;
        flag=sum/10;
        p.next=Some(Box::new(ListNode::new(rst)));
        p=p.next.as_mut().unwrap();
        p1=p1.and_then(|node| node.next.as_ref());
        p2=p2.and_then(|node| node.next.as_ref());

    }
    if flag==1 {
        p.next= Some(Box::new(ListNode::new(1)));
    }
    head.next
}