

// use std::i32;

fn main() {
    assert_eq!(reverse(120),21);
    assert_eq!(reverse(1534236469),0);
}
pub fn reverse(x: i32) -> i32 {
    use std::str::{from_utf8};
    if x==0 { return 0}
    let flag = if x>0 {
        true
    }else{
        false
    };
    let x=x.abs();
    let mut s=x.to_string().into_bytes();
    s.reverse();
    let rst=i32::from_str_radix(from_utf8(&s).unwrap(), 10).unwrap_or_default();
    if flag{
        rst
    }else {
        -rst
    }
    
   
    
}