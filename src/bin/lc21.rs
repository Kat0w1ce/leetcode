use std::{borrow::Borrow, ptr::NonNull};

// #[derive(PartialEq, Eq, Clone, Debug)]
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
type  List=Option<Box<ListNode>>;
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
{


    match (l1, l2) {    // gets of ownership of T in Option<T>
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (Some(n1), Some(n2)) => {
            if n1.val < n2.val {
                Some(n1).map(|mut x| {
                    x.next = merge_two_lists(x.next.take(), Some(n2));
                    x
                })
            } else {
                Some(n2).map(|mut x| {
                    x.next = merge_two_lists(Some(n1), x.next.take());
                    x
                })
            }
        }
        _ => None,
    }
}
// pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     match(l1.is_none(),l2.is_none()) {
//         (true,false)=> return  l2,
//         (false,true)=> return  l1,
//         (false,false)=> return  None,
//         (_,_)=> ()
//     }
//     let  head= ListNode::new(0);
//     let mut p=&head;
    
    
//     let mut a=Some(Box::new(ListNode{val:0,next:l1}));
//     let mut b=Some(Box::new(ListNode{val:0,next:l2}));
//     // while a.unwrap().next.is_some()||b.unwrap().next.is_some() {
//         while let {
//         // let k= match (a.unwrap().next.is_some(),b.unwrap().next.is_some()) {
//         //     (false,true)=>{let tmp=b.unwrap().next.unwrap().val;
//         //                     b=b.unwrap().next;
//         //                     tmp}
//         //     (true,false)=>{let tmp=a.unwrap().next.unwrap().val;
//         //                     a=a.unwrap().next;
//         //                     tmp}
//         //     (true,true)=>{let tmp=a.unwrap().next.unwrap().val

//         //     }
            
//         //     (_,_)=> return head.next
//         // };
//         match (a.unwrap().next,b.unwrap().next) {
//                 (None,Some(node))=>{
                    
//                 }
//                 (Some(node),None)=>{}
//                 (Some(Anode),Some(Bnode))=>{}
//                 (_,_)=>return head.next
            
//         };
//     }
//     None
// }

// impl Iterator for ListNode {
    
//     type Item = Option<i32>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let rst=match self.next {
//             Some(i)
            
//         };

//         None
//     }
// }