// #![feature(int_error_matching)]
// use std::{num::{ParseIntError,IntErrorKind}, vec};




fn main(){
    // println!("{:?}",i32::from_str_radix("+-32", 10));
    assert_eq!(my_atoi(String::from("-91283472332")),i32::MIN);
    assert_eq!(my_atoi(String::from("+-32")),0);
}

pub fn my_atoi(str: String) -> i32 {
    let (n, s) = match str.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
        Some('+') => (1, 1),
        Some(x) if x.is_digit(10) => (0, 1),
        Some('-') => (1, -1),
        _ => return 0,
    };
    let mut res = 0i32;
    let overflow = if s > 0 { std::i32::MAX } else { std::i32::MIN };
    for c in str.chars().skip_while(|x| x.is_whitespace()).skip(n)
        .take_while(|x| x.is_digit(10)) {
            let (r, o) = res.overflowing_mul(10);
            if o { return overflow; }
            let (r, o) = r.overflowing_add(s*(c as i32 - '0' as i32));
            if o { return overflow; }
            res = r;
    }
    res
}
// my solution with unstable
// pub fn my_atoi(str: String) -> i32 {
//     use std::str::from_utf8;
//     let  mut s=str.trim();
//     if s.len()==0 {return 0}
//     let mut flag=false;
//     if s.chars().nth(0).unwrap()== '-'{
//         flag=true;
//     }
//     let mut rst:Vec<u8>=vec![];
//     for c in s.as_bytes() {
//         if c.is_ascii_digit() || *c==b'+' || *c==b'-' {
//             rst.push(*c);
//         } else {
//             break;
//         }
//     }
//     if rst.len()==0{
//         return 0;
//     }
//     let str=from_utf8(&rst).unwrap();
//     let i64=i64::from_str_radix(str, 10);
//     let mut rst=0;
//     match  i64{
//         Ok(i)=>{
//             // rst=i.clamp(i32::MIN as i64,i32::MAX as i64);
//             if i < i32::MIN as i64{
//                 rst=i32::MIN as i64;
//             }else if i> i32:: MIN as i64{
//                 rst=i32::MAX as i64;
//             }else {
//                 rst= i;
//             }
//         },
//        Err(e)=>{
          
//             if *e.kind()== IntErrorKind::InvalidDigit{
//                 rst=0;
//             }else if flag {
//                 rst = i32::MIN as i64;
//             }else {
//                 rst=i32::MAX as i64;
//             }
//         }
//     }
//     rst as i32
// }
// // }